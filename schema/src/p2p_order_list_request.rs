
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_order_list/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// List active orders.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct P2pOrderListRequest {
    /// [Optional] Should be 1 to list active, 0 to list inactive (historical).
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: ActiveEnum,
    /// [Optional] If present, lists orders applying to a specific advert.
    #[serde(rename = "advert_id", skip_serializing_if = "Option::is_none")]
    pub advert_id: String,
    /// [Optional] Filter the orders created after this date(included) format(epoch or YYYY-MM-DD)
    #[serde(rename = "date_from", skip_serializing_if = "Option::is_none")]
    pub date_from: String,
    /// [Optional] Filter the orders created before this date(included) format(epoch or YYYY-MM-DD)
    #[serde(rename = "date_to", skip_serializing_if = "Option::is_none")]
    pub date_to: String,
    /// [Optional] Used for paging.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: i64,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// [Optional] Used for paging.
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: i64,
    /// Must be 1
    #[serde(rename = "p2p_order_list")]
    pub p_2p_order_list: P2pOrderListEnum,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// [Optional] If set to 1, will send updates whenever there is a change to any order belonging to you.
    #[serde(rename = "subscribe", skip_serializing_if = "Option::is_none")]
    pub subscribe: SubscribeEnum,
}




/// [Optional] Should be 1 to list active, 0 to list inactive (historical).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActiveEnum {
    Value0,
    Value1 = 1,
}

impl ActiveEnum {
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
/// Must be 1
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum P2pOrderListEnum {
    Value1 = 1,
}

impl P2pOrderListEnum {
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
