
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/payment_methods/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

// It's a struct
/// Deposit limits for this method.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DepositLimitsValue {
    /// Maximum amount for deposits for this currency.\n
    // Correct serde attribute construction - Use helper
    
    pub max: i64,
    /// Minimum amount for deposit for this currency.\n
    // Correct serde attribute construction - Use helper
    
    pub min: i64,
}

