
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_advertiser_create/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Registers the client as a P2P advertiser.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct P2pAdvertiserCreateRequest {
    /// [Optional] Advertiser's contact information, to be used as a default for new sell adverts.
    #[serde(rename = "contact_info", skip_serializing_if = "Option::is_none")]
    pub contact_info: String,
    /// [Optional] Default description that can be used every time an advert is created.
    #[serde(rename = "default_advert_description", skip_serializing_if = "Option::is_none")]
    pub default_advert_description: String,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// The advertiser's displayed name.
    #[serde(rename = "name")]
    pub name: String,
    /// Must be 1
    #[serde(rename = "p2p_advertiser_create")]
    pub p_2p_advertiser_create: P2pAdvertiserCreateEnum,
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
    /// [Optional] If set to 1, will send updates whenever there is an update to advertiser
    #[serde(rename = "subscribe", skip_serializing_if = "Option::is_none")]
    pub subscribe: SubscribeEnum,
}




/// Must be 1
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum P2pAdvertiserCreateEnum {
    Value1 = 1,
}

impl P2pAdvertiserCreateEnum {
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
