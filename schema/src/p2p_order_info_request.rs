
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_order_info/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Retrieves the information about a P2P order.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct P2pOrderInfoRequest {
    /// The unique identifier for the order.
    #[serde(rename = "id")]
    pub id: String,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// Must be 1
    #[serde(rename = "p2p_order_info")]
    pub p_2p_order_info: P2pOrderInfoEnum,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// [Optional] If set to 1, will send updates whenever there is an update to order
    #[serde(rename = "subscribe", skip_serializing_if = "Option::is_none")]
    pub subscribe: SubscribeEnum,
}




/// Must be 1
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum P2pOrderInfoEnum {
    Value1 = 1,
}

impl P2pOrderInfoEnum {
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
