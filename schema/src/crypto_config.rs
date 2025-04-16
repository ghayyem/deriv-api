
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/crypto_config/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 
use serde_json::Value;
use std::collections::HashMap;


// Import shared types from the *same* crate
use crate::currencies_config_value::CurrenciesConfigValue; 

// It's a struct
/// Provides cryptocurrencies configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CryptoConfig {
    /// Currency configuration including limitiations for each crypto currency.\n
    // Correct serde attribute construction - Use helper
    
    pub currencies_config: HashMap<String, CurrenciesConfigValue>,
}

