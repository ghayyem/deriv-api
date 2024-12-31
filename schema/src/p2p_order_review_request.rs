
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_order_review/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Creates a review for the specified order.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct P2pOrderReviewRequest {
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// The order identification number.
    #[serde(rename = "order_id")]
    pub order_id: String,
    /// Must be 1
    #[serde(rename = "p2p_order_review")]
    pub p_2p_order_review: P2pOrderReviewEnum,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// Rating for the transaction, 1 to 5.
    #[serde(rename = "rating")]
    pub rating: i64,
    /// [Optional] `1` if the counterparty is recommendable to others, otherwise `0`.
    #[serde(rename = "recommended", skip_serializing_if = "Option::is_none")]
    pub recommended: Option<RecommendedEnum>,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
}




/// Must be 1
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum P2pOrderReviewEnum {
    Value1 = 1,
}

impl P2pOrderReviewEnum {
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
/// [Optional] `1` if the counterparty is recommendable to others, otherwise `0`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecommendedEnum {
    Value0,
    Value1 = 1,
}

impl RecommendedEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Value0 => "0",
            Self::Value1 => "1",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "0" => Some(Self::Value0),
            "1" => Some(Self::Value1),
            _ => None,
        }
    }
}
