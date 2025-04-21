use crate::error::{DerivError, Result};
use crate::utils::{validate_app_id, validate_language, validate_schema};
use futures_util::{SinkExt, StreamExt};
use log::debug;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde::de::Error as SerdeError;
use std::future::Future;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot, RwLock};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use url::Url;
use crate::subscription::Subscription;

const DEFAULT_BUFFER_SIZE: usize = 1024;

/// Configuration options for the Deriv API client
#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub keep_alive: bool,
    pub debug: bool,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            keep_alive: false,
            debug: false,
        }
    }
}

/// The main client for interacting with the Deriv API
#[derive(Debug, Clone)]
pub struct DerivClient {
    endpoint: Url,
    origin: Url,
    app_id: i32,
    language: String,
    config: ClientConfig,
    last_request_id: Arc<AtomicI64>,
    request_sender: Option<mpsc::Sender<ApiRequest>>,
    pending_request_registrar: Option<mpsc::Sender<PendingRequestInfo>>,
    connection_status: Arc<RwLock<bool>>,
}

#[derive(Debug)]
struct ApiRequest {
    message: Vec<u8>,
    request_id: i32,
}

#[derive(Debug)]
struct PendingRequestInfo {
    req_id: i32,
    response_sender: oneshot::Sender<Result<Vec<u8>>>,
}

#[derive(Debug, Deserialize)]
struct ApiResponseReqId {
    req_id: Option<i32>,
    error: Option<serde_json::Value>,
}

impl DerivClient {
    /// Creates a new instance of DerivClient
    pub fn new(
        endpoint: &str,
        app_id: i32,
        language: &str,
        origin: &str,
        config: Option<ClientConfig>,
    ) -> Result<Self> {
        let endpoint_url = Url::parse(endpoint)?;
        let origin_url = Url::parse(origin)?;

        validate_schema(&endpoint_url)?;
        validate_app_id(app_id)?;
        validate_language(language)?;

        // Build the endpoint URL with query parameters
        let mut endpoint_url = endpoint_url;
        endpoint_url
            .query_pairs_mut()
            .append_pair("app_id", &app_id.to_string())
            .append_pair("l", language);

        Ok(Self {
            endpoint: endpoint_url,
            origin: origin_url,
            app_id,
            language: language.to_string(),
            config: config.unwrap_or_default(),
            last_request_id: Arc::new(AtomicI64::new(0)),
            request_sender: None,
            pending_request_registrar: None,
            connection_status: Arc::new(RwLock::new(false)),
        })
    }

    /// Connects to the Deriv WebSocket API
    pub async fn connect(&mut self) -> Result<()> {
        if *self.connection_status.read().await {
            return Ok(());
        }

        debug!("Connecting to {}", self.endpoint);

        let ws_stream = connect_async(&self.endpoint).await?.0;
        let (mut write, mut read) = ws_stream.split();

        let (request_sender, mut request_receiver) = mpsc::channel(DEFAULT_BUFFER_SIZE);
        let (incoming_msg_sender, mut incoming_msg_receiver) = mpsc::channel(DEFAULT_BUFFER_SIZE);
        let (pending_request_sender, mut pending_request_receiver) = mpsc::channel::<PendingRequestInfo>(DEFAULT_BUFFER_SIZE);

        self.request_sender = Some(request_sender);
        self.pending_request_registrar = Some(pending_request_sender);
        *self.connection_status.write().await = true;
        let connection_status_write = self.connection_status.clone();
        let _connection_status_read = self.connection_status.clone();

        let _write_task = tokio::spawn(async move {
            while let Some(request) = request_receiver.recv().await {
                let msg_str = String::from_utf8_lossy(&request.message).into_owned();
                debug!("Sending message: {}", msg_str);
                if let Err(e) = write.send(Message::Text(msg_str)).await {
                    debug!("Failed to send message for req_id {}: {}", request.request_id, e);
                    break;
                }
                debug!("Message sent successfully for req_id: {}", request.request_id);
            }
            debug!("Sender task finished.");
        });

        let _read_task = tokio::spawn(async move {
            while let Some(message_result) = read.next().await {
                debug!("Received raw message: {:?}", message_result);
                match message_result {
                    Ok(Message::Text(text)) => {
                        debug!("Received text message: {}", text);
                        if incoming_msg_sender.send(text.into_bytes()).await.is_err() {
                            debug!("Failed to forward response to handler, receiver dropped.");
                            break;
                        }
                    }
                    Ok(Message::Close(close_frame)) => {
                        debug!("Received Close frame: {:?}", close_frame);
                        break;
                    }
                    Ok(Message::Ping(ping_data)) => {
                        debug!("Received Ping: {:?}", ping_data);
                    }
                    Ok(_) => {
                        debug!("Received other message type");
                    }
                    Err(e) => {
                        debug!("WebSocket read error: {}", e);
                        break;
                    }
                }
            }
            debug!("Receiver task finished.");
            *connection_status_write.write().await = false;
        });

        let _response_handler = tokio::spawn(async move {
            let mut pending_requests: std::collections::HashMap<i32, oneshot::Sender<Result<Vec<u8>>>> =
                std::collections::HashMap::new();

            loop {
                tokio::select! {
                    Some(pending_info) = pending_request_receiver.recv() => {
                        debug!("Registering pending request: {}", pending_info.req_id);
                        pending_requests.insert(pending_info.req_id, pending_info.response_sender);
                    }
                    Some(response_bytes) = incoming_msg_receiver.recv() => {
                        match serde_json::from_slice::<ApiResponseReqId>(&response_bytes) {
                            Ok(api_response) => {
                                if let Some(req_id) = api_response.req_id {
                                    if let Some(sender) = pending_requests.remove(&req_id) {
                                        debug!("Routing response for req_id: {}", req_id);
                                        if api_response.error.is_some() {
                                            debug!("API Error found for req_id: {}", req_id);
                                            match crate::error::parse_error(&response_bytes) {
                                                Ok(_) => {
                                                    debug!("Warning: API error flag set, but parse_error succeeded for req_id: {}", req_id);
                                                    let _ = sender.send(Ok(response_bytes));
                                                }
                                                Err(e) => {
                                                    debug!("Sending parsed error for req_id: {}: {:?}", req_id, e);
                                                    let _ = sender.send(Err(e));
                                                }
                                            }
                                        } else {
                                            debug!("Success response for req_id: {}", req_id);
                                            let _ = sender.send(Ok(response_bytes));
                                        }
                                    } else {
                                        debug!("Received response for unknown req_id: {}", req_id);
                                    }
                                } else {
                                    debug!("Received message without req_id (likely subscription): {:?}", String::from_utf8_lossy(&response_bytes));
                                    // Process subscription message
                                    crate::subscription::handle_subscription_message(&response_bytes);
                                }
                            }
                            Err(e) => {
                                debug!("Failed to parse incoming message JSON for req_id: {}", e);
                            }
                        }
                    }
                    else => {
                        debug!("Response handler loop exiting.");
                        break;
                    }
                }
            }
            debug!("Response handler task finished.");
            for (_, sender) in pending_requests.drain() {
                let _ = sender.send(Err(DerivError::ConnectionClosed));
            }
        });

        Ok(())
    }

    /// Disconnects from the Deriv WebSocket API
    pub async fn disconnect(&mut self) {
        if !*self.connection_status.read().await {
            return;
        }

        debug!("Disconnecting from {}", self.endpoint);

        self.request_sender = None;
        self.pending_request_registrar = None;
        *self.connection_status.write().await = false;
    }

    /// Sends a request to the Deriv API and receives a response
    pub async fn send_request<T, R>(&self, request: &T) -> Result<R>
    where
        T: Serialize + std::fmt::Debug,
        R: DeserializeOwned,
    {
        if !*self.connection_status.read().await {
            debug!("Attempted send_request while not connected.");
            return Err(DerivError::ConnectionClosed);
        }

        let request_id = self.get_next_request_id();
        let (response_sender, response_receiver) = oneshot::channel();

        let mut request_value = serde_json::to_value(request)?;
        if let Some(obj) = request_value.as_object_mut() {
            obj.insert("req_id".to_string(), serde_json::json!(request_id));
        } else {
            return Err(DerivError::SerializationError(serde_json::Error::custom("Request is not a JSON object")));
        }
        let message = serde_json::to_vec(&request_value)?;

        debug!("Serialized JSON being sent: {}", String::from_utf8_lossy(&message));

        debug!("Preparing request req_id: {}, payload: {:?}", request_id, request);

        if let Some(registrar) = &self.pending_request_registrar {
            let pending_info = PendingRequestInfo {
                req_id: request_id,
                response_sender,
            };
            if registrar.send(pending_info).await.is_err() {
                debug!("Failed to register pending request {}, response handler likely dead.", request_id);
                return Err(DerivError::ConnectionClosed);
            }
            debug!("Pending request {} registered.", request_id);
        } else {
            debug!("Attempted send_request but registrar is None (not connected?).");
            return Err(DerivError::ConnectionClosed);
        }

        let api_request = ApiRequest {
            message,
            request_id,
        };

        if let Some(sender) = &self.request_sender {
            debug!("Sending request {} to writer task.", request_id);
            if sender.send(api_request).await.is_err() {
                debug!("Failed to send request {} to writer task (channel closed).", request_id);
                return Err(DerivError::ConnectionClosed);
            }
            debug!("Request {} sent to writer task.", request_id);
        } else {
            debug!("Attempted send_request but sender is None (not connected?).");
            return Err(DerivError::ConnectionClosed);
        }

        debug!("Waiting for response for req_id: {}", request_id);
        match response_receiver.await {
            Ok(Ok(response_bytes)) => {
                debug!("Received successful response bytes for req_id: {}", request_id);
                crate::error::parse_error(&response_bytes)?;
                debug!("Deserializing successful response for req_id: {}", request_id);
                Ok(serde_json::from_slice(&response_bytes)?)
            }
            Ok(Err(e)) => {
                debug!("Received error from response handler for req_id: {}: {:?}", request_id, e);
                Err(e)
            }
            Err(_) => {
                debug!("Oneshot channel closed for req_id: {} (handler died?).", request_id);
                Err(DerivError::ConnectionClosed)
            }
        }
    }

    fn get_next_request_id(&self) -> i32 {
        self.last_request_id.fetch_add(1, Ordering::SeqCst) as i32
    }
    
    /// Creates a subscription from a subscription-enabled API
    pub async fn create_subscription<T, R, S>(&self, request: &mut T, msg_type: &str) -> Result<(R, Subscription<S>)>
    where
        T: Serialize + std::fmt::Debug,
        R: DeserializeOwned + Serialize,
        S: DeserializeOwned + Send + 'static,
    {
        if !*self.connection_status.read().await {
            debug!("Attempted create_subscription while not connected.");
            return Err(DerivError::ConnectionClosed);
        }
        
        // Send the initial request to start the subscription
        let initial_response: R = self.send_request(request).await?;
        
        // Extract the subscription ID from the response
        let response_value = serde_json::to_vec(&initial_response)?;
        let subscription_id = crate::subscription::parse_subscription_response(&response_value)?;
        
        // Create a channel for the subscription updates
        let (_sender, receiver) = mpsc::channel::<S>(100);
        
        // Create the subscription with an Arc to self
        let client_arc = Arc::new(self.clone());
        let subscription = Subscription::new(receiver, subscription_id, client_arc, msg_type);
        
        Ok((initial_response, subscription))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_creation() {
        let client = DerivClient::new(
            "wss://ws.binaryws.com/websockets/v3",
            1234,
            "en",
            "https://binary.com",
            None,
        );
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_invalid_schema() {
        let client = DerivClient::new(
            "http://ws.binaryws.com/websockets/v3",
            1234,
            "en",
            "https://binary.com",
            None,
        );
        assert!(matches!(client, Err(DerivError::InvalidSchema(_))));
    }

    #[tokio::test]
    async fn test_invalid_app_id() {
        let client = DerivClient::new(
            "wss://ws.binaryws.com/websockets/v3",
            0,
            "en",
            "https://binary.com",
            None,
        );
        assert!(matches!(client, Err(DerivError::InvalidAppId(_))));
    }

    #[tokio::test]
    async fn test_invalid_language() {
        let client = DerivClient::new(
            "wss://ws.binaryws.com/websockets/v3",
            1234,
            "eng",
            "https://binary.com",
            None,
        );
        assert!(matches!(client, Err(DerivError::InvalidLanguage(_))));
    }
}

trait ReceiverExt<T> {
    fn recv_next(&mut self) -> impl Future<Output = Option<T>>;
}

impl<T> ReceiverExt<T> for mpsc::Receiver<T> {
    async fn recv_next(&mut self) -> Option<T> {
        self.recv().await
    }
}

