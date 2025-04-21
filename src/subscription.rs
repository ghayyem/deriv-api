use crate::error::{DerivError, Result};
use futures_util::Stream;
use log::{debug, error, warn};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use tokio::sync::mpsc;

// Global subscription registry to track active subscriptions
lazy_static::lazy_static! {
    static ref SUBSCRIPTION_REGISTRY: Arc<Mutex<SubscriptionRegistry>> =
        Arc::new(Mutex::new(SubscriptionRegistry::new()));
}

// Registry to map subscription IDs to subscription channels
struct SubscriptionRegistry {
    subscriptions: HashMap<String, SubscriptionSender>,
    msg_type_map: HashMap<String, String>, // Maps msg_type to subscription_id
}

impl SubscriptionRegistry {
    fn new() -> Self {
        Self {
            subscriptions: HashMap::new(),
            msg_type_map: HashMap::new(),
        }
    }

    fn register<T>(&mut self, subscription_id: String, sender: mpsc::Sender<T>, msg_type: &str) 
    where 
        T: DeserializeOwned + Send + 'static 
    {
        let sender_box = Box::new(move |data: &[u8]| {
            match serde_json::from_slice::<T>(data) {
                Ok(parsed) => {
                    // Try sending the parsed data
                    if sender.try_send(parsed).is_err() {
                        debug!("Failed to send subscription update - receiver dropped");
                        return false; // Return false to indicate we should remove this subscription
                    }
                    true
                }
                Err(e) => {
                    error!("Failed to parse subscription data: {}", e);
                    true // Keep subscription even if parsing fails
                }
            }
        }) as SubscriptionSender;

        self.subscriptions.insert(subscription_id.clone(), sender_box);
        self.msg_type_map.insert(msg_type.to_string(), subscription_id.clone());
        
        debug!("Registered subscription: {} for msg_type: {}", subscription_id, msg_type);
    }

    fn dispatch(&mut self, data: &[u8]) -> bool {
        // Extract the subscription ID or msg_type from the data
        if let Ok(json) = serde_json::from_slice::<Value>(data) {
            // First try to find subscription ID directly
            if let Some(id) = json.get("id").and_then(|v| v.as_str()).or_else(|| {
                // If not, look for subscription.id
                json.get("subscription")
                    .and_then(|s| s.get("id"))
                    .and_then(|id| id.as_str())
            }) {
                if let Some(sender) = self.subscriptions.get_mut(id) {
                    debug!("Found subscription handler for ID: {}", id);
                    return sender(data);
                }
            }

            // If no ID found or no handler for that ID, try using msg_type
            if let Some(msg_type) = json.get("msg_type").and_then(|v| v.as_str()) {
                if let Some(subscription_id) = self.msg_type_map.get(msg_type) {
                    if let Some(sender) = self.subscriptions.get_mut(subscription_id) {
                        debug!("Found subscription handler for msg_type: {}", msg_type);
                        return sender(data);
                    }
                }
            }
        }
        
        debug!("No handler found for subscription update");
        true // Keep checking future messages
    }

    fn unregister(&mut self, subscription_id: &str) -> bool {
        // Find and remove any msg_type mappings for this subscription
        let msg_types_to_remove: Vec<String> = self.msg_type_map
            .iter()
            .filter_map(|(msg_type, id)| {
                if id == subscription_id {
                    Some(msg_type.clone())
                } else {
                    None
                }
            })
            .collect();
            
        for msg_type in msg_types_to_remove {
            self.msg_type_map.remove(&msg_type);
        }
        
        // Remove the subscription itself
        self.subscriptions.remove(subscription_id).is_some()
    }
}

// Type for storing subscription senders that can handle any type
type SubscriptionSender = Box<dyn FnMut(&[u8]) -> bool + Send>;

// Public function to handle incoming subscription messages
pub(crate) fn handle_subscription_message(data: &[u8]) {
    let mut registry = SUBSCRIPTION_REGISTRY.lock().unwrap();
    if !registry.dispatch(data) {
        // If dispatch returns false, the receiver was dropped, so remove the subscription
        if let Ok(json) = serde_json::from_slice::<Value>(data) {
            if let Some(id) = json.get("id")
                .and_then(|v| v.as_str())
                .or_else(|| json.get("subscription")
                    .and_then(|s| s.get("id"))
                    .and_then(|id| id.as_str())) 
            {
                debug!("Removing dropped subscription: {}", id);
                registry.unregister(id);
            }
        }
    }
}

pub struct Subscription<T> {
    receiver: mpsc::Receiver<T>,
    subscription_id: String,
    client: Arc<crate::client::DerivClient>,
}

impl<T> Subscription<T> 
where
    T: DeserializeOwned + Send + 'static
{
    pub(crate) fn new(
        receiver: mpsc::Receiver<T>, 
        subscription_id: String,
        client: Arc<crate::client::DerivClient>,
        msg_type: &str
    ) -> Self {
        // Register the subscription in the registry
        let (tx, _) = mpsc::channel::<T>(100); // Create a dummy sender just for type inference
        SUBSCRIPTION_REGISTRY.lock().unwrap().register(
            subscription_id.clone(),
            tx, // This won't be used since we're providing the receiver directly
            msg_type
        );
        
        Self {
            receiver,
            subscription_id,
            client,
        }
    }

    pub fn subscription_id(&self) -> &str {
        &self.subscription_id
    }

    pub async fn forget(&mut self) -> Result<()> {
        // Send a forget request to the server
        let forget_request = deriv_api_schema::ForgetRequest {
            forget: self.subscription_id.clone(),
            passthrough: None,
            req_id: None,
        };
        
        // Send the forget request
        let _forget_response = self.client.forget(forget_request).await?;
        
        // Remove from local registry
        SUBSCRIPTION_REGISTRY.lock().unwrap().unregister(&self.subscription_id);
        
        Ok(())
    }
}

impl<T> Stream for Subscription<T> {
    type Item = T;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.receiver).poll_recv(cx)
    }
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct SubscriptionResponse {
    pub subscription: Option<SubscriptionInfo>,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct SubscriptionInfo {
    pub id: String,
}

pub(crate) fn parse_subscription_response(response: &[u8]) -> Result<String> {
    let subscription_response: SubscriptionResponse = serde_json::from_slice(response)?;

    subscription_response
        .subscription
        .map(|s| s.id)
        .ok_or(DerivError::EmptySubscriptionId)
}
