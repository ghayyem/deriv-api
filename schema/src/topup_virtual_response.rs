
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/topup_virtual/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// The result of virtual money top up
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct TopupVirtualResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// The information regarding a successful top up for a virtual money account
    #[serde(rename = "topup_virtual", skip_serializing_if = "Option::is_none")]
    pub topup_virtual: TopupVirtual,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// The information regarding a successful top up for a virtual money account
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct TopupVirtual {
    /// Top up amount
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: f64,
    /// Top up currency string
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: String,
}






