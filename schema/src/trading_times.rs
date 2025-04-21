
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/trading_times/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::market_item::MarketItem; 

// It's a struct
/// The trading times structure is a hierarchy as follows: Market -> SubMarket -> Underlyings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TradingTimes {
    /// An array of markets\n
    // Correct serde attribute construction - Use helper
    
    pub markets: Vec<MarketItem>,
}

