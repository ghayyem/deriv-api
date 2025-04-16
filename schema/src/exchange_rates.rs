
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/exchange_rates/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 

use std::collections::HashMap;
use chrono::{DateTime, Utc};

// Import shared types from the *same* crate

// It's a struct
/// Exchange rate values from base to target currency
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExchangeRates {
    /// Base currency\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub base_currency: Option<String>,
    /// Date retrieval epoch time represented as an integer number\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub date: Option<DateTime<Utc>>,
    /// Rate of exchanging a unit of base currency into a target currency\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub rates: Option<HashMap<String, f64>>,
}

