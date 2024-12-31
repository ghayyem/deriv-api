
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/profit_table/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// A summary of account profit table is received
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct ProfitTableResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Account Profit Table.
    #[serde(rename = "profit_table", skip_serializing_if = "Option::is_none")]
    pub profit_table: ProfitTable,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Account Profit Table.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct ProfitTable {
    /// Number of transactions returned in this call
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: f64,
    /// Array of returned transactions
    #[serde(rename = "transactions", skip_serializing_if = "Option::is_none")]
    pub transactions: Vec<Transactionsitem>,
}






