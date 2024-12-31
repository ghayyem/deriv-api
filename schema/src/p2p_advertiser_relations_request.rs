
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_advertiser_relations/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Updates and returns favourite and blocked advertisers of the current user.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct P2pAdvertiserRelationsRequest {
    /// IDs of advertisers to block.
    #[serde(rename = "add_blocked", skip_serializing_if = "Option::is_none")]
    pub add_blocked: Vec<f64>,
    /// IDs of advertisers to add as favourites.
    #[serde(rename = "add_favourites", skip_serializing_if = "Option::is_none")]
    pub add_favourites: Vec<f64>,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// Must be 1
    #[serde(rename = "p2p_advertiser_relations")]
    pub p_2p_advertiser_relations: P2pAdvertiserRelationsEnum,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// IDs of advertisers to remove from blocked.
    #[serde(rename = "remove_blocked", skip_serializing_if = "Option::is_none")]
    pub remove_blocked: Vec<f64>,
    /// IDs of advertisers to remove from favourites.
    #[serde(rename = "remove_favourites", skip_serializing_if = "Option::is_none")]
    pub remove_favourites: Vec<f64>,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
}




/// Must be 1
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum P2pAdvertiserRelationsEnum {
    Value1 = 1,
}

impl P2pAdvertiserRelationsEnum {
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
