
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/states_list/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// A message with States List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct StatesListResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// List of states.
    #[serde(rename = "states_list", skip_serializing_if = "Option::is_none")]
    pub states_list: Vec<StatesListitem>,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct StatesListitem {
    /// The state name.
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: String,
    /// The state code.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: String,
}






