
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/states_list/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

// It's a struct
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StatesListItem {
    /// The state name.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub text: Option<String>,
    /// The state code.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub value: Option<String>,
}

