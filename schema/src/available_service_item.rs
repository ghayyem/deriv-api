
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/kyc_auth_status/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

// It's a struct
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AvailableServiceItem {
    /// The specific document type name expected by the service.\n
    // Correct serde attribute construction - Use helper
    
    pub document_type: String,
    /// The name of the service supporting this document type.\n
    // Correct serde attribute construction - Use helper
    
    pub service: String,
}

