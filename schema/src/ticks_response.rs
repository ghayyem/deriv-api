
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/ticks/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Latest spot price for a given symbol. Continuous responses with a frequency of up to one second.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct TicksResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Type of the response.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// For subscription requests only.
    #[serde(rename = "subscription", skip_serializing_if = "Option::is_none")]
    pub subscription: Subscription,
    /// Tick by tick list of streamed data
    #[serde(rename = "tick", skip_serializing_if = "Option::is_none")]
    pub tick: Tick,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Tick by tick list of streamed data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Tick {
    /// Market ask at the epoch
    #[serde(rename = "ask", skip_serializing_if = "Option::is_none")]
    pub ask: f64,
    /// Market bid at the epoch
    #[serde(rename = "bid", skip_serializing_if = "Option::is_none")]
    pub bid: f64,
    /// Epoch time of the tick
    #[serde(rename = "epoch", skip_serializing_if = "Option::is_none")]
    pub epoch: i64,
    /// A per-connection unique identifier. Can be passed to the `forget` API call to unsubscribe.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    /// Indicates the number of decimal points that the returned amounts must be displayed with
    #[serde(rename = "pip_size")]
    pub pip_size: f64,
    /// Market value at the epoch
    #[serde(rename = "quote", skip_serializing_if = "Option::is_none")]
    pub quote: f64,
    /// Symbol
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: String,
}






