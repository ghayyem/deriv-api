
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/document_upload/send.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 
use serde_json::Value;



// Import shared types from the *same* crate

// It's a struct
/// [Optional] It contains info about the proof of ownership being uploaded (mandatory for proof_of_ownership document type)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProofOfOwnership {
    /// A collection of unspecific information related to the proof of ownership being uploaded\n
    // Correct serde attribute construction - Use helper
    
    pub details: Value,
    /// The id of the proof of ownership as shown in the /get_account_status proof of ownership list\n
    // Correct serde attribute construction - Use helper
    
    pub id: f64,
}

