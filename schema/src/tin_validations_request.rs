
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/tin_validations/send.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


// Import required types from the *same* crate
use crate::tin_validations_enum::TinValidationsEnum;

/// Get the validations for Tax Identification Numbers (TIN)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TinValidationsRequest {
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub passthrough: Option<Value>,
    /// [Optional] Used to map request to response.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub req_id: Option<i64>,
    /// The tax residence selected by the client.\n
    // Correct serde attribute construction - Use helper
    
    pub tax_residence: String,
    /// Must be `1`\n
    // Correct serde attribute construction - Use helper
    
    pub tin_validations: TinValidationsEnum,
}

