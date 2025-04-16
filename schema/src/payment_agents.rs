
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/website_config/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 

use std::collections::HashMap;


// Import shared types from the *same* crate

// It's a struct
/// Payments Agents system settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PaymentAgents {
    /// Initial deposit requirement per country.\n
    // Correct serde attribute construction - Use helper
    
    pub initial_deposit_per_country: HashMap<String, f64>,
}

