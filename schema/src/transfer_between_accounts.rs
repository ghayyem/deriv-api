
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/website_config/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 
use serde_json::Value;
use std::collections::HashMap;


// Import shared types from the *same* crate

// It's a struct
/// Fees and range of allowed amount for transfer between accounts with different types of currencies.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TransferBetweenAccounts {
    /// The fee that applies for transfer between accounts with different types of currencies.\n
    // Correct serde attribute construction - Use helper
    
    pub fees: HashMap<String, f64>,
    /// Range of allowed amount for transfer between accounts.\n
    // Correct serde attribute construction - Use helper
    
    pub limits: Value,
    /// Range of allowed amount for transfer between ctrader accounts.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub limits_ctrader: Option<Value>,
    /// Range of allowed amount for transfer between dxtrade accounts.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub limits_dxtrade: Option<Value>,
    /// Range of allowed amount for transfer between mt5 accounts.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub limits_mt5: Option<Value>,
}

