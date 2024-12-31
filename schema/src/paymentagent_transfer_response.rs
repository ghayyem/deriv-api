
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/paymentagent_transfer/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// The result of transfer request made.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct PaymentagentTransferResponse {
    /// The `transfer_to` client full name
    #[serde(rename = "client_to_full_name", skip_serializing_if = "Option::is_none")]
    pub client_to_full_name: String,
    /// The `transfer_to` client loginid
    #[serde(rename = "client_to_loginid", skip_serializing_if = "Option::is_none")]
    pub client_to_loginid: String,
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// If set to `1`, transfer success. If set to `2`, dry-run success.
    #[serde(rename = "paymentagent_transfer", skip_serializing_if = "Option::is_none")]
    pub paymentagent_transfer: PaymentagentTransferEnum,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// Reference ID of transfer performed
    #[serde(rename = "transaction_id", skip_serializing_if = "Option::is_none")]
    pub transaction_id: i64,
}




