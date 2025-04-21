
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/trading_durations/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::data_item::DataItem; 
use crate::market::Market; 
use crate::submarket::Submarket; 

// It's a struct
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TradingDurationItem {
    /// Available contract types and trading duration boundaries\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub data: Option<Vec<DataItem>>,
    /// The market in which the underlyings listed in `symbol` located.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub market: Option<Market>,
    /// The submarket in which the underlyings listed in `symbol` located.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub submarket: Option<Submarket>,
}

