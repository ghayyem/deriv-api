
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_order_cancel/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Result of the P2P order cancellation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct P2pOrderCancelResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Cancellation details
    #[serde(rename = "p2p_order_cancel", skip_serializing_if = "Option::is_none")]
    pub p_2p_order_cancel: P2pOrderCancel,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Cancellation details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct P2pOrderCancel {
    /// The unique identifier for the order.
    #[serde(rename = "id")]
    pub id: String,
    /// The new status of the order.
    #[serde(rename = "status")]
    pub status: StatusEnum,
}






