
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/paymentagent_withdraw/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// The result of payment agent withdrawal request made.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct PaymentagentWithdrawResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Payment agent name.
    #[serde(rename = "paymentagent_name", skip_serializing_if = "Option::is_none")]
    pub paymentagent_name: String,
    /// If set to `1`, withdrawal success. If set to `2`, dry-run success.
    #[serde(rename = "paymentagent_withdraw", skip_serializing_if = "Option::is_none")]
    pub paymentagent_withdraw: PaymentagentWithdrawEnum,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// Reference ID of withdrawal performed.
    #[serde(rename = "transaction_id", skip_serializing_if = "Option::is_none")]
    pub transaction_id: i64,
}




