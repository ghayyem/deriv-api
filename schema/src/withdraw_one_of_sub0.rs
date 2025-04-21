
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/cashier/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

// It's a struct
/// Result for withdraw operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WithdrawOneOfSub0 {
    /// The unique identifier for the transaction.\n
    // Correct serde attribute construction - Use helper
    
    pub id: String,
    /// The status code of the withdrawal.\n
    // Correct serde attribute construction - Use helper
    
    pub status_code: String,
    /// The status message of the withdrawal.\n
    // Correct serde attribute construction - Use helper
    
    pub status_message: String,
}

