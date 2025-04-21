
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/exchange_verification_code/send.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;



// Import required types from the *same* crate

/// Verifies the code and returns a new code if verification is successful
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExchangeVerificationCodeRequest {
    /// Must be `1`\n
    // Correct serde attribute construction - Use helper
    
    pub exchange_verification_code: i64,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub passthrough: Option<Value>,
    /// [Optional] Used to map request to response.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub req_id: Option<i64>,
    /// [Optional] Purpose of the token exchange.\n
    // Correct serde attribute construction - Use helper
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")] 
    pub type_: Option<String>,
    /// Email verification code (received from a `verify_email` call which must be done first).\n
    // Correct serde attribute construction - Use helper
    
    pub verification_code: String,
}

