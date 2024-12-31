
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/buy/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// A message with transaction results is received
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct BuyResponse {
    /// Receipt confirmation for the purchase
    #[serde(rename = "buy", skip_serializing_if = "Option::is_none")]
    pub buy: Buy,
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
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



/// Receipt confirmation for the purchase
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Buy {
    /// The new account balance after completion of the purchase
    #[serde(rename = "balance_after")]
    pub balance_after: f64,
    /// Actual effected purchase price
    #[serde(rename = "buy_price")]
    pub buy_price: f64,
    /// Internal contract identifier
    #[serde(rename = "contract_id")]
    pub contract_id: i64,
    /// The description of contract purchased
    #[serde(rename = "longcode")]
    pub longcode: String,
    /// Proposed payout value
    #[serde(rename = "payout")]
    pub payout: f64,
    /// Epoch value of the transaction purchase time
    #[serde(rename = "purchase_time")]
    pub purchase_time: i64,
    /// Compact description of the contract purchased
    #[serde(rename = "shortcode")]
    pub shortcode: String,
    /// Epoch value showing the expected start time of the contract
    #[serde(rename = "start_time")]
    pub start_time: i64,
    /// Internal transaction identifier
    #[serde(rename = "transaction_id")]
    pub transaction_id: i64,
}






