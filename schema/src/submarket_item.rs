
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/trading_times/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::symbol_item::SymbolItem; 

// It's a struct
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SubmarketItem {
    /// Submarket name\n
    // Correct serde attribute construction - Use helper
    
    pub name: String,
    /// Symbols array\n
    // Correct serde attribute construction - Use helper
    
    pub symbols: Vec<SymbolItem>,
}

