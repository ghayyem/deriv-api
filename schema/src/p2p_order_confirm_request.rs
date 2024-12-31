
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_order_confirm/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Confirm a P2P order.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct P2pOrderConfirmRequest {
    /// [Optional] If set to `1`, only validation is performed.
    #[serde(rename = "dry_run", skip_serializing_if = "Option::is_none")]
    pub dry_run: DryRunEnum,
    /// The unique identifier for this order.
    #[serde(rename = "id")]
    pub id: String,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// Must be 1
    #[serde(rename = "p2p_order_confirm")]
    pub p_2p_order_confirm: P2pOrderConfirmEnum,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// [Optional] Verification code received from email.
    #[serde(rename = "verification_code", skip_serializing_if = "Option::is_none")]
    pub verification_code: String,
}




/// Must be 1
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum P2pOrderConfirmEnum {
    Value1 = 1,
}

impl P2pOrderConfirmEnum {
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
