
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/exchange_rates/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Retrieves the exchange rate from a base currency to a target currency supported by the system.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct ExchangeRatesRequest {
    /// Base currency (can be obtained from `payout_currencies` call)
    #[serde(rename = "base_currency")]
    pub base_currency: String,
    /// Must be `1`
    #[serde(rename = "exchange_rates")]
    pub exchange_rates: ExchangeRatesEnum,
    /// [Optional] 1 - Request for ask and bid rates along with the spot rate. Only available if target_currency is provided.
    #[serde(rename = "include_spread", skip_serializing_if = "Option::is_none")]
    pub include_spread: IncludeSpreadEnum,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// [Optional] 1 - to initiate a realtime stream of exchange rates relative to base currency.
    #[serde(rename = "subscribe", skip_serializing_if = "Option::is_none")]
    pub subscribe: SubscribeEnum,
    /// [Optional] Target currency for the exchange rate. If subscribe is set, target_currency must be specified as well.
    #[serde(rename = "target_currency", skip_serializing_if = "Option::is_none")]
    pub target_currency: String,
}




/// Must be `1`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExchangeRatesEnum {
    Value1 = 1,
}

impl ExchangeRatesEnum {
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
/// [Optional] 1 - Request for ask and bid rates along with the spot rate. Only available if target_currency is provided.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IncludeSpreadEnum {
    Value1 = 1,
}

impl IncludeSpreadEnum {
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
