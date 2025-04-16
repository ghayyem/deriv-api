
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/trading_times/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 
use serde_json::Value;
use std::collections::HashMap;


// Import shared types from the *same* crate
use crate::trading_days_item_enum::TradingDaysItemEnum; 

// It's a struct
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SymbolsItem {
    /// Events\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub events: Option<Vec<Value>>,
    /// Symbol name\n
    // Correct serde attribute construction - Use helper
    
    pub name: String,
    /// Symbol shortcode\n
    // Correct serde attribute construction - Use helper
    
    pub symbol: String,
    /// Open, close and settlement times\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub times: Option<Value>,
    /// Trading days\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub trading_days: Option<Vec<TradingDaysItemEnum>>,
}

