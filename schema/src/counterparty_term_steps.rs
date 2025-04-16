
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_settings/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

// It's a struct
/// Recommended step values for choosing advert counterparty terms.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CounterpartyTermSteps {
    /// Values for minimum 30 day completion rate.\n
    // Correct serde attribute construction - Use helper
    
    pub completion_rate: Vec<f64>,
    /// Values for minimum joined days.\n
    // Correct serde attribute construction - Use helper
    
    pub join_days: Vec<i64>,
    /// Values for minimum average rating.\n
    // Correct serde attribute construction - Use helper
    
    pub rating: Vec<f64>,
}

