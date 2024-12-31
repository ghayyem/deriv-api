
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/exchange_rates/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// The exchange rate values from the specified base currency to the specified target currency supported by the system.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct ExchangeRatesResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Exchange rate values from base to target currency
    #[serde(rename = "exchange_rates", skip_serializing_if = "Option::is_none")]
    pub exchange_rates: ExchangeRates,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// For subscription requests only.
    #[serde(rename = "subscription", skip_serializing_if = "Option::is_none")]
    pub subscription: Subscription,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Exchange rate values from base to target currency
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct ExchangeRates {
    /// Base currency
    #[serde(rename = "base_currency", skip_serializing_if = "Option::is_none")]
    pub base_currency: String,
    /// Date retrieval epoch time represented as an integer number
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: i64,
    /// Rate of exchanging a unit of base currency into a target currency
    #[serde(rename = "rates", skip_serializing_if = "Option::is_none", flatten)]
    pub rates: HashMap<String, Value>,
}






