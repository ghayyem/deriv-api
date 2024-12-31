
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_advertiser_payment_methods/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Manage or list P2P advertiser payment methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct P2pAdvertiserPaymentMethodsRequest {
    /// Contains new payment method entries.
    #[serde(rename = "create", skip_serializing_if = "Option::is_none")]
    pub create: Vec<HashMap<String, String>>,
    /// Contains payment methods to delete.
    #[serde(rename = "delete", skip_serializing_if = "Option::is_none")]
    pub delete: Vec<f64>,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// Must be 1
    #[serde(rename = "p2p_advertiser_payment_methods")]
    pub p_2p_advertiser_payment_methods: P2pAdvertiserPaymentMethodsEnum,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// Contains payment methods to update.
    #[serde(rename = "update", skip_serializing_if = "Option::is_none", flatten)]
    pub update: HashMap<String, HashMap<String, String>>,
}




/// Must be 1
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum P2pAdvertiserPaymentMethodsEnum {
    Value1 = 1,
}

impl P2pAdvertiserPaymentMethodsEnum {
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
