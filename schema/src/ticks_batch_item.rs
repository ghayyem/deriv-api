
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/ticks_batch/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

// It's a struct
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TicksBatchItem {
    /// Market ask at the epoch\n
    // Correct serde attribute construction - Use helper
    
    pub ask: String,
    /// Market bid at the epoch\n
    // Correct serde attribute construction - Use helper
    
    pub bid: String,
    /// Daily percentage change\n
    // Correct serde attribute construction - Use helper
    
    pub change: String,
    /// Epoch time of the tick\n
    // Correct serde attribute construction - Use helper
    
    pub epoch: i64,
    /// Market value at the epoch\n
    // Correct serde attribute construction - Use helper
    
    pub quote: String,
    /// Symbol\n
    // Correct serde attribute construction - Use helper
    
    pub symbol: String,
}

