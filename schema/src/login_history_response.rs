
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/login_history/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Recent login/logout history records
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct LoginHistoryResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Array of records of client login/logout activities
    #[serde(rename = "login_history", skip_serializing_if = "Option::is_none")]
    pub login_history: Vec<LoginHistoryitem>,
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



/// User login history
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct LoginHistoryitem {
    /// Type of action.
    #[serde(rename = "action")]
    pub action: String,
    /// Provides details about browser, device used during login or logout
    #[serde(rename = "environment")]
    pub environment: String,
    /// Status of activity: 1 - success, 0 - failure
    #[serde(rename = "status")]
    pub status: StatusEnum,
    /// Epoch time of the activity
    #[serde(rename = "time")]
    pub time: i64,
}






