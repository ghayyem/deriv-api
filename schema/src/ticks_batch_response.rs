
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/ticks_batch/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Latest spot price for a group symbols. Continuous responses with a frequency of up to one second.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct TicksBatchResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// For subscription requests only.
    #[serde(rename = "subscription", skip_serializing_if = "Option::is_none")]
    pub subscription: Subscription,
    /// Tick by tick list of streamed data for a group of symbols
    #[serde(rename = "ticks_batch", skip_serializing_if = "Option::is_none")]
    pub ticks_batch: Vec<TicksBatchitem>,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct TicksBatchitem {
    /// Market ask at the epoch
    #[serde(rename = "ask")]
    pub ask: String,
    /// Market bid at the epoch
    #[serde(rename = "bid")]
    pub bid: String,
    /// Daily percentage change
    #[serde(rename = "change")]
    pub change: String,
    /// Epoch time of the tick
    #[serde(rename = "epoch")]
    pub epoch: i64,
    /// Market value at the epoch
    #[serde(rename = "quote")]
    pub quote: String,
    /// Symbol
    #[serde(rename = "symbol")]
    pub symbol: String,
}






