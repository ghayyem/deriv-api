
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_advert_update/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Updates a P2P advert. Can only be used by the advertiser.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct P2pAdvertUpdateRequest {
    /// [Optional] Advertiser contact information.
    #[serde(rename = "contact_info", skip_serializing_if = "Option::is_none")]
    pub contact_info: String,
    /// [Optional] If set to 1, permanently deletes the advert.
    #[serde(rename = "delete", skip_serializing_if = "Option::is_none")]
    pub delete: DeleteEnum,
    /// [Optional] General information about the advert.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: String,
    /// [Optional] 2 letter country codes. Counterparties who do not live in these countries will not be allowed to place orders against this advert. An empty array or null value will clear the condition.
    #[serde(rename = "eligible_countries", skip_serializing_if = "Option::is_none")]
    pub eligible_countries: Option<Value>,
    /// The unique identifier for this advert.
    #[serde(rename = "id")]
    pub id: String,
    /// [Optional] Activate or deactivate the advert.
    #[serde(rename = "is_active", skip_serializing_if = "Option::is_none")]
    pub is_active: IsActiveEnum,
    /// [Optional] Local currency for this advert.
    #[serde(rename = "local_currency", skip_serializing_if = "Option::is_none")]
    pub local_currency: String,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// [Optional] Maximum allowed amount for the orders of this advert, in advertiser's `account_currency`. Should be more than or equal to `min_order_amount`.
    #[serde(rename = "max_order_amount", skip_serializing_if = "Option::is_none")]
    pub max_order_amount: f64,
    /// [Optional] Counterparties who have a 30 day completion rate less than this value will not be allowed to place orders against this advert. A an empty array or null value will clear the condition.
    #[serde(rename = "min_completion_rate", skip_serializing_if = "Option::is_none")]
    pub min_completion_rate: Option<Value>,
    /// [Optional] Counterparties who joined less than this number of days ago will not be allowed to place orders against this advert. A null value will clear the condition.
    #[serde(rename = "min_join_days", skip_serializing_if = "Option::is_none")]
    pub min_join_days: Option<Value>,
    /// [Optional] Minimum allowed amount for the orders of this advert, in advertiser's `account_currency`. Should be less than or equal to `max_order_amount`.
    #[serde(rename = "min_order_amount", skip_serializing_if = "Option::is_none")]
    pub min_order_amount: f64,
    /// [Optional] Counterparties who have an average rating less than this value will not be allowed to place orders against this advert. A null value will clear the condition.
    #[serde(rename = "min_rating", skip_serializing_if = "Option::is_none")]
    pub min_rating: Option<Value>,
    /// [Optional] Expiry period (seconds) for order created against this ad.
    #[serde(rename = "order_expiry_period", skip_serializing_if = "Option::is_none")]
    pub order_expiry_period: i64,
    /// Must be 1
    #[serde(rename = "p2p_advert_update")]
    pub p_2p_advert_update: P2pAdvertUpdateEnum,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] Payment instructions.
    #[serde(rename = "payment_info", skip_serializing_if = "Option::is_none")]
    pub payment_info: String,
    /// [Optional] IDs of previously saved payment methods as returned from p2p_advertiser_payment_methods, only applicable for sell ads. Exisiting methods will be replaced.
    #[serde(rename = "payment_method_ids", skip_serializing_if = "Option::is_none")]
    pub payment_method_ids: Vec<i64>,
    /// [Optional] Payment method identifiers as returned from p2p_payment_methods, only applicable for buy ads. Exisiting methods will be replaced.
    #[serde(rename = "payment_method_names", skip_serializing_if = "Option::is_none")]
    pub payment_method_names: Vec<String>,
    /// [Optional] Conversion rate from advertiser's account currency to `local_currency`. An absolute rate value (fixed), or percentage offset from current market rate (floating).
    #[serde(rename = "rate", skip_serializing_if = "Option::is_none")]
    pub rate: f64,
    /// [Optional] Type of rate, fixed or floating.
    #[serde(rename = "rate_type", skip_serializing_if = "Option::is_none")]
    pub rate_type: RateTypeEnum,
    /// [Optional] The total available amount of the advert, in advertiser's account currency.
    #[serde(rename = "remaining_amount", skip_serializing_if = "Option::is_none")]
    pub remaining_amount: f64,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
}




/// [Optional] If set to 1, permanently deletes the advert.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeleteEnum {
    Value0,
    Value1 = 1,
}

impl DeleteEnum {
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
pub enum P2pAdvertUpdateEnum {
    Value1 = 1,
}

impl P2pAdvertUpdateEnum {
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
