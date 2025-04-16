
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_advertiser_info/send.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


// Import required types from the *same* crate
use crate::subscribe_enum::SubscribeEnum;
use crate::p2p_advertiser_info_enum::P2pAdvertiserInfoEnum;

/// Retrieve information about a P2P advertiser.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct P2pAdvertiserInfoRequest {
    /// [Optional] The unique identifier for this advertiser. If not provided, returns advertiser information about the current account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub id: Option<String>,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub loginid: Option<String>,
    /// Must be 1\n
    // Correct serde attribute construction - Use helper
    
    pub p2p_advertiser_info: P2pAdvertiserInfoEnum,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub passthrough: Option<Value>,
    /// [Optional] Used to map request to response.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub req_id: Option<i64>,
    /// [Optional] If set to 1, will send updates whenever there is an update to advertiser\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub subscribe: Option<SubscribeEnum>,
}

