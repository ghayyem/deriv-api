
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_advert_create/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Creates a P2P (Peer to Peer) advert. Can only be used by an approved P2P advertiser.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct P2pAdvertCreateRequest {
    /// The total amount of the advert, in advertiser's account currency.
    #[serde(rename = "amount")]
    pub amount: f64,
    /// [Optional] Indicates if this is block trade ad or not. Default: 0.
    #[serde(rename = "block_trade", skip_serializing_if = "Option::is_none")]
    pub block_trade: BlockTradeEnum,
    /// [Optional] Advertiser contact information.
    #[serde(rename = "contact_info", skip_serializing_if = "Option::is_none")]
    pub contact_info: String,
    /// [Optional] General information about the advert.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: String,
    /// [Optional] 2 letter country codes. Counterparties who do not live in these countries will not be allowed to place orders against the advert.
    #[serde(rename = "eligible_countries", skip_serializing_if = "Option::is_none")]
    pub eligible_countries: Vec<String>,
    /// [Optional] Local currency for this advert. If not provided, will use the currency of client's residence by default.
    #[serde(rename = "local_currency", skip_serializing_if = "Option::is_none")]
    pub local_currency: String,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// Maximum allowed amount for the orders of this advert, in advertiser's `account_currency`. Should be more than or equal to `min_order_amount`
    #[serde(rename = "max_order_amount")]
    pub max_order_amount: f64,
    /// [Optional] Counterparties who have a 30 day completion rate less than this value will not be allowed to place orders against the advert.
    #[serde(rename = "min_completion_rate", skip_serializing_if = "Option::is_none")]
    pub min_completion_rate: f64,
    /// [Optional] Counterparties who joined less than this number of days ago will not be allowed to place orders against the advert.
    #[serde(rename = "min_join_days", skip_serializing_if = "Option::is_none")]
    pub min_join_days: i64,
    /// Minimum allowed amount for the orders of this advert, in advertiser's `account_currency`. Should be less than or equal to `max_order_amount`.
    #[serde(rename = "min_order_amount")]
    pub min_order_amount: f64,
    /// [Optional] Counterparties who have an average rating less than this value will not be allowed to place orders against the advert.
    #[serde(rename = "min_rating", skip_serializing_if = "Option::is_none")]
    pub min_rating: f64,
    /// [Optional] Expiry period (seconds) for order created against this ad.
    #[serde(rename = "order_expiry_period", skip_serializing_if = "Option::is_none")]
    pub order_expiry_period: i64,
    /// Must be 1
    #[serde(rename = "p2p_advert_create")]
    pub p_2p_advert_create: P2pAdvertCreateEnum,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] Payment instructions.
    #[serde(rename = "payment_info", skip_serializing_if = "Option::is_none")]
    pub payment_info: String,
    /// [Optional] Payment method name (deprecated).
    #[serde(rename = "payment_method", skip_serializing_if = "Option::is_none")]
    pub payment_method: String,
    /// IDs of previously saved payment methods as returned from p2p_advertiser_payment_methods, only applicable for sell ads.
    #[serde(rename = "payment_method_ids", skip_serializing_if = "Option::is_none")]
    pub payment_method_ids: Vec<i64>,
    /// Payment method identifiers as returned from p2p_payment_methods, only applicable for buy ads.
    #[serde(rename = "payment_method_names", skip_serializing_if = "Option::is_none")]
    pub payment_method_names: Vec<String>,
    /// Conversion rate from advertiser's account currency to `local_currency`. An absolute rate value (fixed), or percentage offset from current market rate (floating).
    #[serde(rename = "rate")]
    pub rate: f64,
    /// Type of rate, fixed or floating.
    #[serde(rename = "rate_type", skip_serializing_if = "Option::is_none")]
    pub rate_type: RateTypeEnum,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// The advertisement represents the intention to perform this action on your Deriv account funds.
    #[serde(rename = "type")]
    pub type: TypeEnum,
}




/// Must be 1
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum P2pAdvertCreateEnum {
    Value1 = 1,
}

impl P2pAdvertCreateEnum {
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
/// Type of rate, fixed or floating.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RateTypeEnum {
    Fixed,
    Float,
}

impl RateTypeEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Fixed => "fixed",
            Self::Float => "float",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "fixed" => Some(Self::Fixed),
            "float" => Some(Self::Float),
            _ => None,
        }
    }
}
