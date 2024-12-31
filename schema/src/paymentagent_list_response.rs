
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/paymentagent_list/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// A message with Payment Agent List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct PaymentagentListResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Payment Agent List
    #[serde(rename = "paymentagent_list", skip_serializing_if = "Option::is_none")]
    pub paymentagent_list: PaymentagentList,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Payment Agent List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct PaymentagentList {
    /// The list of countries in which payment agent is available.
    #[serde(rename = "available_countries", skip_serializing_if = "Option::is_none")]
    pub available_countries: Vec<Vec<Value>>,
    /// List of payment agents available in the requested country.
    #[serde(rename = "list")]
    pub list: Vec<Listitem>,
}






