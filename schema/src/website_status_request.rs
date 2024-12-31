
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/website_status/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Request server status.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct WebsiteStatusRequest {
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// [Optional] `1` to stream the server/website status updates.
    #[serde(rename = "subscribe", skip_serializing_if = "Option::is_none")]
    pub subscribe: SubscribeEnum,
    /// Must be `1`
    #[serde(rename = "website_status")]
    pub website_status: WebsiteStatusEnum,
}




/// Must be `1`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WebsiteStatusEnum {
    Value1 = 1,
}

impl WebsiteStatusEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Value1 => "1",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "1" => Some(Self::Value1),
            _ => None,
        }
    }
}
