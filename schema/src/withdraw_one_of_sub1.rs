
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/cashier/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

// It's a struct
/// Result for `dry_run` validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WithdrawOneOfSub1 {
    /// The `dry_run` was successful.\n
    // Correct serde attribute construction - Use helper
    
    pub dry_run: i64,
}

