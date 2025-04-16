
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/website_status/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

// It's a struct
/// Maximum number of general requests allowed during specified period of time.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MaxRequestesGeneral {
    /// Describes which calls this limit applies to.\n
    // Correct serde attribute construction - Use helper
    
    pub applies_to: String,
    /// The maximum of allowed calls per hour.\n
    // Correct serde attribute construction - Use helper
    
    pub hourly: f64,
    /// The maximum of allowed calls per minute.\n
    // Correct serde attribute construction - Use helper
    
    pub minutely: f64,
}

