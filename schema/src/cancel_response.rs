
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/cancel/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// A message with transaction results is received
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct CancelResponse {
    /// Receipt for the transaction
    #[serde(rename = "cancel", skip_serializing_if = "Option::is_none")]
    pub cancel: Cancel,
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Receipt for the transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Cancel {
    /// New account balance after completion of the sale
    #[serde(rename = "balance_after", skip_serializing_if = "Option::is_none")]
    pub balance_after: f64,
    /// Internal contract identifier for the sold contract
    #[serde(rename = "contract_id", skip_serializing_if = "Option::is_none")]
    pub contract_id: i64,
    /// Internal transaction identifier for the corresponding buy transaction
    #[serde(rename = "reference_id", skip_serializing_if = "Option::is_none")]
    pub reference_id: i64,
    /// Actual effected sale price
    #[serde(rename = "sold_for", skip_serializing_if = "Option::is_none")]
    pub sold_for: f64,
    /// Internal transaction identifier for the sale transaction
    #[serde(rename = "transaction_id", skip_serializing_if = "Option::is_none")]
    pub transaction_id: i64,
}






