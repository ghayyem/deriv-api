
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/portfolio/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Receive a list of outstanding options in the user's portfolio
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct PortfolioResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Current account's open positions.
    #[serde(rename = "portfolio", skip_serializing_if = "Option::is_none")]
    pub portfolio: Portfolio,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Current account's open positions.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Portfolio {
    /// List of open positions.
    #[serde(rename = "contracts")]
    pub contracts: Vec<Contractsitem>,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// The details of each open position.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Contractsitem {
    /// ID of the application where this contract was purchased.
    #[serde(rename = "app_id", skip_serializing_if = "Option::is_none")]
    pub app_id: Option<Value>,
    /// Buy price
    #[serde(rename = "buy_price", skip_serializing_if = "Option::is_none")]
    pub buy_price: f64,
    /// Internal contract identifier number (to be used in a `proposal_open_contract` API call).
    #[serde(rename = "contract_id", skip_serializing_if = "Option::is_none")]
    pub contract_id: i64,
    /// Contract type
    #[serde(rename = "contract_type", skip_serializing_if = "Option::is_none")]
    pub contract_type: String,
    /// Contract currency
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: String,
    /// Epoch of start date
    #[serde(rename = "date_start", skip_serializing_if = "Option::is_none")]
    pub date_start: i64,
    /// Epoch of expiry time
    #[serde(rename = "expiry_time", skip_serializing_if = "Option::is_none")]
    pub expiry_time: i64,
    /// Contract description
    #[serde(rename = "longcode", skip_serializing_if = "Option::is_none")]
    pub longcode: String,
    /// Payout price
    #[serde(rename = "payout", skip_serializing_if = "Option::is_none")]
    pub payout: f64,
    /// Epoch of purchase time
    #[serde(rename = "purchase_time", skip_serializing_if = "Option::is_none")]
    pub purchase_time: i64,
    /// Contract short code description
    #[serde(rename = "shortcode", skip_serializing_if = "Option::is_none")]
    pub shortcode: String,
    /// Symbol code
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: String,
    /// It is the transaction ID. Every contract (buy or sell) and every payment has a unique ID.
    #[serde(rename = "transaction_id", skip_serializing_if = "Option::is_none")]
    pub transaction_id: i64,
}








