use crate::error::{DerivError, Result};
use crate::utils::{validate_app_id, validate_language, validate_schema};
use futures_util::{SinkExt, StreamExt};
use log::debug;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use url::Url;

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
#[derive(Debug)]
pub struct DerivClient {
    endpoint: Url,
    origin: Url,
    app_id: i32,
    language: String,
    config: ClientConfig,
    last_request_id: Arc<AtomicI64>,
    request_sender: Option<mpsc::Sender<ApiRequest>>,
    connection_status: Arc<RwLock<bool>>,
}

#[derive(Debug)]
struct ApiRequest {
    message: Vec<u8>,
    response_sender: mpsc::Sender<Vec<u8>>,
    request_id: i32,
}

#[derive(Debug, Deserialize)]
struct ApiResponseReqId {
    req_id: Option<i32>,
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
        let (write, read) = ws_stream.split();

        let (request_sender, request_receiver) = mpsc::channel(DEFAULT_BUFFER_SIZE);
        let (response_sender, mut response_receiver) = mpsc::channel(DEFAULT_BUFFER_SIZE);

        self.request_sender = Some(request_sender);
        *self.connection_status.write().await = true;

        // Spawn message sender task
        let write_task = tokio::spawn(async move {
            let mut write = write;
            let mut request_receiver = request_receiver; // Make mutable here
            while let Some(request) = request_receiver.recv().await {
                if let Err(e) = write
                    .send(Message::Text(
                        String::from_utf8_lossy(&request.message).into_owned(),
                    ))
                    .await
                {
                    debug!("Failed to send message: {}", e);
                    break;
                }
            }
        });

        // Spawn message receiver task
        let read_task = tokio::spawn(async move {
            let mut read = read;
            while let Some(Ok(message)) = read.next().await {
                match message {
                    Message::Text(text) => {
                        if let Err(e) = response_sender.send(text.into_bytes()).await {
                            debug!("Failed to forward response: {}", e);
                            break;
                        }
                    }
                    Message::Close(_) => break,
                    _ => continue,
                }
            }
        });

        // Spawn response handler task
        let response_handler = tokio::spawn(async move {
            let mut response_map: std::collections::HashMap<i32, mpsc::Sender<Vec<u8>>> =
                std::collections::HashMap::new();

            while let Some(response) = response_receiver.recv().await {
                if let Ok(api_response) = serde_json::from_slice::<ApiResponseReqId>(&response) {
                    if let Some(req_id) = api_response.req_id {
                        if let Some(sender) = response_map.remove(&req_id) {
                            let _ = sender.send(response).await;
                        }
                    }
                }
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
        *self.connection_status.write().await = false;
    }

    /// Sends a request to the Deriv API and receives a response
    pub async fn send_request<T, R>(&self, request: &T) -> Result<R>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let request_id = self.get_next_request_id();
        let (response_sender, mut response_receiver) = mpsc::channel(1);

        let mut request_value = serde_json::to_value(request)?;
        if let Some(obj) = request_value.as_object_mut() {
            obj.insert("req_id".to_string(), serde_json::json!(request_id));
        }

        let message = serde_json::to_vec(&request_value)?;

        let api_request = ApiRequest {
            message,
            response_sender,
            request_id,
        };

        if let Some(sender) = &self.request_sender {
            sender
                .send(api_request)
                .await
                .map_err(|_| DerivError::ConnectionClosed)?;
        } else {
            return Err(DerivError::ConnectionClosed);
        }

        let response = response_receiver
            .recv()
            .await
            .ok_or(DerivError::ConnectionClosed)?;

        crate::error::parse_error(&response)?;
        Ok(serde_json::from_slice(&response)?)
    }

    fn get_next_request_id(&self) -> i32 {
        self.last_request_id.fetch_add(1, Ordering::SeqCst) as i32
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
