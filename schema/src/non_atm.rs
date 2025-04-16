
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/get_limits/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

// It's a struct
/// Maximum aggregate payouts on open positions per symbol for contract where barrier is different from entry spot.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NonAtm {
    /// Maximum aggregate payouts on open positions per symbol for contract where barrier is different from entry spot and duration is less than and equal to seven days\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub less_than_seven_days: Option<f64>,
    /// Maximum aggregate payouts on open positions per symbol for contract where barrier is different from entry spot and duration is more to seven days\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub more_than_seven_days: Option<f64>,
}

