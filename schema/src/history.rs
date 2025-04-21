
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/ticks_history/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

// It's a struct
/// Historic tick data for a given symbol. Note: this will always return the latest possible set of ticks with accordance to the parameters specified.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct History {
    /// An array containing list of tick values for the corresponding epoch values in `times` array.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub prices: Option<String>,
    /// An array containing list of epoch values for the corresponding tick values in `prices` array.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub times: Option<String>,
}

