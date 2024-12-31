use crate::error::{DerivError, Result};
use futures_util::Stream;
use serde::de::DeserializeOwned;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::sync::mpsc;

pub struct Subscription<T> {
    receiver: mpsc::Receiver<T>,
    subscription_id: String,
}

impl<T> Subscription<T> {
    pub(crate) fn new(receiver: mpsc::Receiver<T>, subscription_id: String) -> Self {
        Self {
            receiver,
            subscription_id,
        }
    }

    pub fn subscription_id(&self) -> &str {
        &self.subscription_id
    }

    pub async fn forget(&mut self) -> Result<()> {
        // Implementation for forget functionality
        // This will be implemented when we add the full API functionality
        Ok(())
    }
}

impl<T> Stream for Subscription<T> {
    type Item = T;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.receiver).poll_recv(cx)
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct SubscriptionResponse {
    pub subscription: Option<SubscriptionInfo>,
}

#[derive(serde::Deserialize)]
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
