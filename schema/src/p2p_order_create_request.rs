
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_order_create/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Creates a P2P order for the specified advert.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct P2pOrderCreateRequest {
    /// The unique identifier for the advert to create an order against.
    #[serde(rename = "advert_id")]
    pub advert_id: String,
    /// The amount of currency to be bought or sold.
    #[serde(rename = "amount")]
    pub amount: f64,
    /// [Optional] Seller contact information. Only applicable for 'sell orders'.
    #[serde(rename = "contact_info", skip_serializing_if = "Option::is_none")]
    pub contact_info: String,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// Must be 1
    #[serde(rename = "p2p_order_create")]
    pub p_2p_order_create: P2pOrderCreateEnum,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] Payment instructions, only applicable for sell orders.
    #[serde(rename = "payment_info", skip_serializing_if = "Option::is_none")]
    pub payment_info: String,
    /// IDs of payment methods, only applicable for sell orders.
    #[serde(rename = "payment_method_ids", skip_serializing_if = "Option::is_none")]
    pub payment_method_ids: Vec<i64>,
    /// [Optional] Conversion rate from account currency to local currency, only applicable for floating rate adverts.
    #[serde(rename = "rate", skip_serializing_if = "Option::is_none")]
    pub rate: f64,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// [Optional] If set to 1, will send updates whenever there is an update to the order.
    #[serde(rename = "subscribe", skip_serializing_if = "Option::is_none")]
    pub subscribe: SubscribeEnum,
}




/// Must be 1
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum P2pOrderCreateEnum {
    Value1 = 1,
}

impl P2pOrderCreateEnum {
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
/// [Optional] If set to 1, will send updates whenever there is an update to the order.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscribeEnum {
    Value1 = 1,
}

impl SubscribeEnum {
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
