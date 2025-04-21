
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/buy/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

// It's a struct
/// Receipt confirmation for the purchase
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Buy {
    /// The new account balance after completion of the purchase\n
    // Correct serde attribute construction - Use helper
    
    pub balance_after: f64,
    /// Actual effected purchase price\n
    // Correct serde attribute construction - Use helper
    
    pub buy_price: String,
    /// Internal contract identifier\n
    // Correct serde attribute construction - Use helper
    
    pub contract_id: i64,
    /// The description of contract purchased\n
    // Correct serde attribute construction - Use helper
    
    pub longcode: String,
    /// Proposed payout value\n
    // Correct serde attribute construction - Use helper
    
    pub payout: String,
    /// Epoch value of the transaction purchase time\n
    // Correct serde attribute construction - Use helper
    
    pub purchase_time: String,
    /// Compact description of the contract purchased\n
    // Correct serde attribute construction - Use helper
    
    pub shortcode: String,
    /// Epoch value showing the expected start time of the contract\n
    // Correct serde attribute construction - Use helper
    
    pub start_time: String,
    /// Internal transaction identifier\n
    // Correct serde attribute construction - Use helper
    
    pub transaction_id: i64,
}

