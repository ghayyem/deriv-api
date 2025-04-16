
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/website_config/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 
use serde_json::Value;
use std::collections::HashMap;


// Import shared types from the *same* crate

// It's a struct
/// List of cashier platforms supported for this currency. It is categorized by cashier and ramp (on-ramp, off-ramp) platforms.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Platform {
    /// Supported platforms for the cashier, this is passed to provider attribute of `cashier` call\n
    // Correct serde attribute construction - Use helper
    
    pub cashier: Vec<Value>,
    /// Supported platforms for the ramp (on-ramp, off-ramp)\n
    // Correct serde attribute construction - Use helper
    
    pub ramp: Vec<Value>,
}

