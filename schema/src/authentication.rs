
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/get_account_status/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::document::Document; 
use crate::identity::Identity; 
use crate::attempts::Attempts; 
use crate::ownership::Ownership; 
use crate::income::Income; 

// It's a struct
/// This represents the authentication status of the user and it includes what authentication is needed.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Authentication {
    /// POI attempts made by the client\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub attempts: Option<Attempts>,
    /// The authentication status for document.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub document: Option<Document>,
    /// The authentication status for identity.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub identity: Option<Identity>,
    /// The authentication status for source of income document.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub income: Option<Income>,
    /// An array containing the list of required authentication.\n
    // Correct serde attribute construction - Use helper
    
    pub needs_verification: Vec<f64>,
    /// The current state of the proof of ownership.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub ownership: Option<Ownership>,
}

