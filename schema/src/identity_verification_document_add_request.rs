
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/identity_verification_document_add/send.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;



// Import required types from the *same* crate

/// Adds document information such as issuing country, id and type for identity verification processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IdentityVerificationDocumentAddRequest {
    /// [Optional] Additional info required by some document types.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub document_additional: Option<String>,
    /// The identification number of the document.\n
    // Correct serde attribute construction - Use helper
    
    pub document_number: String,
    /// The type of the document based on provided `issuing_country` (can obtained from `residence_list` call).\n
    // Correct serde attribute construction - Use helper
    
    pub document_type: String,
    /// Must be `1`\n
    // Correct serde attribute construction - Use helper
    
    pub identity_verification_document_add: i64,
    /// 2-letter country code (can obtained from `residence_list` call).\n
    // Correct serde attribute construction - Use helper
    
    pub issuing_country: String,
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

