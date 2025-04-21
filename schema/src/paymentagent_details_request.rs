
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/paymentagent_details/send.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;



// Import required types from the *same* crate

/// Gets client's payment agent details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PaymentagentDetailsRequest {
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub loginid: Option<String>,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub passthrough: Option<Value>,
    /// Field 'paymentagent_details' mapped to Value due to complexity/potential issues.\n
    // Correct serde attribute construction - Use helper
    
    pub paymentagent_details: Value,
    /// [Optional] Used to map request to response.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub req_id: Option<i64>,
}

