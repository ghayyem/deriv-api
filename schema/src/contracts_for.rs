
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/contracts_for/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 
use serde_json::Value;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

// Import shared types from the *same* crate
use crate::available_item::AvailableItem; 

// It's a struct
/// List of available contracts. Note: if the user is authenticated, then only contracts allowed under his account will be returned.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ContractsFor {
    /// Array of available contracts details\n
    // Correct serde attribute construction - Use helper
    
    pub available: Vec<AvailableItem>,
    /// Symbol's next market-close time as an epoch value\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub close: Option<DateTime<Utc>>,
    /// Indicates the feed license for symbol, for example whether its realtime or delayed\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub feed_license: Option<String>,
    /// Count of contracts available\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub hit_count: Option<f64>,
    /// Array of non_available contracts details\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub non_available: Option<Vec<Value>>,
    /// Symbol's next market-open time as an epoch value\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub open: Option<DateTime<Utc>>,
    /// Current spot price for this underlying\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub spot: Option<f64>,
}

