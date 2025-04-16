
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/unsubscribe_email/send.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


// Import required types from the *same* crate
use crate::unsubscribe_email_enum::UnsubscribeEmailEnum;

/// It unsubscribe user from the email subscription.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UnsubscribeEmailRequest {
    /// Customer User ID.\n
    // Correct serde attribute construction - Use helper
    
    pub binary_user_id: f64,
    /// The generated checksum for the customer.\n
    // Correct serde attribute construction - Use helper
    
    pub checksum: String,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub passthrough: Option<Value>,
    /// [Optional] Used to map request to response.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub req_id: Option<i64>,
    /// Must be `1`\n
    // Correct serde attribute construction - Use helper
    
    pub unsubscribe_email: UnsubscribeEmailEnum,
}

