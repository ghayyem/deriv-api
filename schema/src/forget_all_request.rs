
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/forget_all/send.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;



// Import required types from the *same* crate

/// Immediately cancel the real-time streams of messages of given type.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ForgetAllRequest {
    /// Cancel all streams by type. The value can be either a single type e.g. `"ticks"`, or an array of multiple types e.g. `["candles", "ticks"]`.\n
    // Correct serde attribute construction - Use helper
    
    pub forget_all: Value,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub passthrough: Option<Value>,
    /// [Optional] Used to map request to response.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub req_id: Option<i64>,
}

