
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/mt5_deposit/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// The result of MT5 deposit request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Mt5DepositResponse {
    /// Withdrawal reference ID of Binary account
    #[serde(rename = "binary_transaction_id", skip_serializing_if = "Option::is_none")]
    pub binary_transaction_id: i64,
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// 1 on success
    #[serde(rename = "mt5_deposit", skip_serializing_if = "Option::is_none")]
    pub mt_5_deposit: i64,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
}




