
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_advertiser_update/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

// It's a struct
/// Block trading limits, if block trading is allowed.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BlockTrade {
    /// Maximum order amount for block trade adverts.\n
    // Correct serde attribute construction - Use helper
    
    pub max_order_amount: f64,
    /// Minimum order amount for block trade adverts.\n
    // Correct serde attribute construction - Use helper
    
    pub min_order_amount: f64,
}

