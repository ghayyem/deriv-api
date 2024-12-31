
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/tnc_approval/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// The result of T&C approval request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct TncApprovalResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// Set terms and conditions 1: success
    #[serde(rename = "tnc_approval", skip_serializing_if = "Option::is_none")]
    pub tnc_approval: TncApprovalEnum,
}




