
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/balance/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

// It's a struct
/// Total balance of all real money Deriv accounts.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Deriv {
    /// Total of balances.\n
    // Correct serde attribute construction - Use helper
    
    pub amount: f64,
    /// Currency of total.\n
    // Correct serde attribute construction - Use helper
    
    pub currency: String,
}

