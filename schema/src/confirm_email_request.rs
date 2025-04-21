
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/confirm_email/send.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;



// Import required types from the *same* crate
use crate::created_for::CreatedFor;

/// Verifies the email for the user using verification code passed in the request object
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ConfirmEmailRequest {
    /// Must be `1`\n
    // Correct serde attribute construction - Use helper
    
    pub confirm_email: i64,
    /// [Optional] Purpose of the email verification. If set to 'account_opening', the API will only return the verification response without updating the user's email verification status.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub created_for: Option<CreatedFor>,
    /// Boolean value: 1 or 0, indicating whether the client has given consent for marketing emails.\n
    // Correct serde attribute construction - Use helper
    
    pub email_consent: String,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub passthrough: Option<Value>,
    /// [Optional] Used to map request to response.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub req_id: Option<i64>,
    /// Email verification code (received from a `verify_email` call, which must be done first).\n
    // Correct serde attribute construction - Use helper
    
    pub verification_code: String,
}

