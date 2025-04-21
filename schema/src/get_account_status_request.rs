
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/get_account_status/send.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;



// Import required types from the *same* crate

/// Get Account Status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetAccountStatusRequest {
    /// Field 'get_account_status' mapped to Value due to complexity/potential issues.\n
    // Correct serde attribute construction - Use helper
    
    pub get_account_status: Value,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub loginid: Option<String>,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub passthrough: Option<Value>,
    /// [Optional] Used to map request to response.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub req_id: Option<i64>,
}

