
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/contracts_for/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Get the list of currently available contracts
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct ContractsForResponse {
    /// List of available contracts. Note: if the user is authenticated, then only contracts allowed under his account will be returned.
    #[serde(rename = "contracts_for", skip_serializing_if = "Option::is_none")]
    pub contracts_for: ContractsFor,
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



/// List of available contracts. Note: if the user is authenticated, then only contracts allowed under his account will be returned.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct ContractsFor {
    /// Array of available contracts details
    #[serde(rename = "available")]
    pub available: Vec<Availableitem>,
    /// Symbol's next market-close time as an epoch value
    #[serde(rename = "close", skip_serializing_if = "Option::is_none")]
    pub close: Option<Value>,
    /// Indicates the feed license for symbol, for example whether its realtime or delayed
    #[serde(rename = "feed_license", skip_serializing_if = "Option::is_none")]
    pub feed_license: String,
    /// Count of contracts available
    #[serde(rename = "hit_count", skip_serializing_if = "Option::is_none")]
    pub hit_count: f64,
    /// Array of non_available contracts details
    #[serde(rename = "non_available", skip_serializing_if = "Option::is_none")]
    pub non_available: Vec<Value>,
    /// Symbol's next market-open time as an epoch value
    #[serde(rename = "open", skip_serializing_if = "Option::is_none")]
    pub open: Option<Value>,
    /// Current spot price for this underlying
    #[serde(rename = "spot", skip_serializing_if = "Option::is_none")]
    pub spot: Option<Value>,
}






