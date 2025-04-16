
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/website_status/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

// It's a struct
/// Maximum subscription to proposal calls.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MaxProposalSubscription {
    /// Describes which calls this limit applies to.\n
    // Correct serde attribute construction - Use helper
    
    pub applies_to: String,
    /// Maximum number of allowed calls.\n
    // Correct serde attribute construction - Use helper
    
    pub max: f64,
}

