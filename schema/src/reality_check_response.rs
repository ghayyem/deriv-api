
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/reality_check/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// This gives summary of client's trades and account for reality check
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct RealityCheckResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Reality check summary of trades.
    #[serde(rename = "reality_check", skip_serializing_if = "Option::is_none")]
    pub reality_check: RealityCheck,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;

use chrono::{DateTime, Utc};

/// Reality check summary of trades.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct RealityCheck {
    /// Total amount of contract purchased.
    #[serde(rename = "buy_amount", skip_serializing_if = "Option::is_none")]
    pub buy_amount: f64,
    /// Total count of contract purchased.
    #[serde(rename = "buy_count", skip_serializing_if = "Option::is_none")]
    pub buy_count: i64,
    /// Currency of client account i.e currency for trading
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: String,
    /// Client loginid.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// Total count of contracts that are not yet expired.
    #[serde(rename = "open_contract_count", skip_serializing_if = "Option::is_none")]
    pub open_contract_count: i64,
    /// Indicative profit of contract as per current market price.
    #[serde(rename = "potential_profit", skip_serializing_if = "Option::is_none")]
    pub potential_profit: f64,
    /// Total amount of contracts sold.
    #[serde(rename = "sell_amount", skip_serializing_if = "Option::is_none")]
    pub sell_amount: f64,
    /// Total count of contract sold.
    #[serde(rename = "sell_count", skip_serializing_if = "Option::is_none")]
    pub sell_count: i64,
    /// Reality check summary start time epoch
    #[serde(rename = "start_time", skip_serializing_if = "Option::is_none")]
    pub start_time: i64,
}






