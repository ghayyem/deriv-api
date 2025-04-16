
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/paymentagent_list/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 
use serde_json::Value;
use std::collections::HashMap;


// Import shared types from the *same* crate

// It's a struct
/// Payment Agent List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PaymentagentList {
    /// The list of countries in which payment agent is available.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub available_countries: Option<Vec<Vec<Option<String>>>>,
    /// Field 'list' mapped to Value due to complexity/potential issues.\n
    // Correct serde attribute construction - Use helper
    
    pub list: Value,
}

