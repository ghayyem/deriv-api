
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/contract_update_history/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Contract update history status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct ContractUpdateHistoryResponse {
    /// Contains the historical and the most recent update status of the contract
    #[serde(rename = "contract_update_history", skip_serializing_if = "Option::is_none")]
    pub contract_update_history: Vec<ContractUpdateHistoryitem>,
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

use chrono::{DateTime, Utc};

/// Contains the changed parameter.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct ContractUpdateHistoryitem {
    /// Display name of the changed parameter.
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: String,
    /// The amount.
    #[serde(rename = "order_amount", skip_serializing_if = "Option::is_none")]
    pub order_amount: String,
    /// The epoch when the changed was done.
    #[serde(rename = "order_date", skip_serializing_if = "Option::is_none")]
    pub order_date: i64,
    /// The contract parameter updated.
    #[serde(rename = "order_type", skip_serializing_if = "Option::is_none")]
    pub order_type: String,
    /// The pip-sized barrier value.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
}






