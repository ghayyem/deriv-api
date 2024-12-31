
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_advertiser_list/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Retrieve advertisers has/had trade with the current advertiser.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct P2pAdvertiserListRequest {
    /// [Optional] Search for advertiser by name. Partial matches will be returned.
    #[serde(rename = "advertiser_name", skip_serializing_if = "Option::is_none")]
    pub advertiser_name: String,
    /// [Optional] Used to return only blocked or unblocked partners
    #[serde(rename = "is_blocked", skip_serializing_if = "Option::is_none")]
    pub is_blocked: IsBlockedEnum,
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
    #[serde(rename = "p2p_advertiser_list")]
    pub p_2p_advertiser_list: P2pAdvertiserListEnum,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// [Optional] How the results are sorted.
    #[serde(rename = "sort_by", skip_serializing_if = "Option::is_none")]
    pub sort_by: SortByEnum,
    /// [Optional] Get all advertisers has/had trade.
    #[serde(rename = "trade_partners", skip_serializing_if = "Option::is_none")]
    pub trade_partners: TradePartnersEnum,
}




/// [Optional] Used to return only blocked or unblocked partners
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IsBlockedEnum {
    Value0,
    Value1 = 1,
}

impl IsBlockedEnum {
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
pub enum P2pAdvertiserListEnum {
    Value1 = 1,
}

impl P2pAdvertiserListEnum {
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
/// [Optional] Get all advertisers has/had trade.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TradePartnersEnum {
    Value0,
    Value1 = 1,
}

impl TradePartnersEnum {
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
