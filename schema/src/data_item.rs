
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/trading_durations/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::market::Market; 
use crate::symbol_item::SymbolItem; 
use crate::submarket::Submarket; 
use crate::trade_durations_item::TradeDurationsItem; 

// It's a struct
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DataItem {
    /// The market in which the underlyings listed in `symbol` located.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub market: Option<Market>,
    /// The submarket in which the underlyings listed in `symbol` located.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub submarket: Option<Submarket>,
    /// List of underlying symbols.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub symbol: Option<Vec<SymbolItem>>,
    /// List of trade durations available for symbols and contract combinations.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub trade_durations: Option<Vec<TradeDurationsItem>>,
}

