
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_advertiser_adverts/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Returns all P2P adverts created by the authorized client. Can only be used by a registered P2P advertiser.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct P2pAdvertiserAdvertsRequest {
    /// [Optional] Used for paging. This value will also apply to subsription responses.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: i64,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// [Optional] Used for paging. This value will also apply to subsription responses.
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: i64,
    /// Must be 1
    #[serde(rename = "p2p_advertiser_adverts")]
    pub p_2p_advertiser_adverts: P2pAdvertiserAdvertsEnum,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
}




/// Must be 1
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum P2pAdvertiserAdvertsEnum {
    Value1 = 1,
}

impl P2pAdvertiserAdvertsEnum {
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
