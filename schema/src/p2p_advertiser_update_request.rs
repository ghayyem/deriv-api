
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_advertiser_update/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Update the information of the P2P advertiser for the current account. Can only be used by an approved P2P advertiser.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct P2pAdvertiserUpdateRequest {
    /// [Optional] Advertiser's contact information, to be used as a default for new sell adverts.
    #[serde(rename = "contact_info", skip_serializing_if = "Option::is_none")]
    pub contact_info: String,
    /// [Optional] Default description that can be used every time an advert is created.
    #[serde(rename = "default_advert_description", skip_serializing_if = "Option::is_none")]
    pub default_advert_description: String,
    /// [Optional] Used to set if the advertiser's adverts could be listed. When `0`, adverts won't be listed regardless of they are active or not. This doesn't change the `is_active` of each individual advert.
    #[serde(rename = "is_listed", skip_serializing_if = "Option::is_none")]
    pub is_listed: IsListedEnum,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// Must be 1
    #[serde(rename = "p2p_advertiser_update")]
    pub p_2p_advertiser_update: P2pAdvertiserUpdateEnum,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] Advertiser's payment information, to be used as a default for new sell adverts.
    #[serde(rename = "payment_info", skip_serializing_if = "Option::is_none")]
    pub payment_info: String,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// [Optional] Weekly availability schedule. Ads are visible and orders can be created only during available periods.
    #[serde(rename = "schedule", skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Value>,
    /// [Optional] When `1`, the advertiser's real name will be displayed on to other users on adverts and orders.
    #[serde(rename = "show_name", skip_serializing_if = "Option::is_none")]
    pub show_name: ShowNameEnum,
    /// [Optional] Used to upgrade daily limits of eligible advertiser.
    #[serde(rename = "upgrade_limits", skip_serializing_if = "Option::is_none")]
    pub upgrade_limits: UpgradeLimitsEnum,
}




/// Must be 1
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum P2pAdvertiserUpdateEnum {
    Value1 = 1,
}

impl P2pAdvertiserUpdateEnum {
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
/// [Optional] Used to upgrade daily limits of eligible advertiser.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UpgradeLimitsEnum {
    Value1 = 1,
}

impl UpgradeLimitsEnum {
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
