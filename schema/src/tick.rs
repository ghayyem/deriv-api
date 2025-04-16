
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/ticks/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 


use chrono::{DateTime, Utc};

// Import shared types from the *same* crate

// It's a struct
/// Tick by tick list of streamed data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Tick {
    /// Market ask at the epoch\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub ask: Option<f64>,
    /// Market bid at the epoch\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub bid: Option<f64>,
    /// Epoch time of the tick\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub epoch: Option<i64>,
    /// A per-connection unique identifier. Can be passed to the `forget` API call to unsubscribe.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub id: Option<String>,
    /// Indicates the number of decimal points that the returned amounts must be displayed with\n
    // Correct serde attribute construction - Use helper
    
    pub pip_size: f64,
    /// Market value at the epoch\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub quote: Option<f64>,
    /// Symbol\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub symbol: Option<String>,
}

