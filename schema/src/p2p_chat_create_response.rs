
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_chat_create/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Information of the created P2P chat.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct P2pChatCreateResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Information of the P2P chat.
    #[serde(rename = "p2p_chat_create", skip_serializing_if = "Option::is_none")]
    pub p_2p_chat_create: P2pChatCreate,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Information of the P2P chat.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct P2pChatCreate {
    /// The URL to be used to initialise the chat for the requested order.
    #[serde(rename = "channel_url")]
    pub channel_url: String,
    /// The unique identifier for the order that the chat belongs to.
    #[serde(rename = "order_id")]
    pub order_id: String,
}






