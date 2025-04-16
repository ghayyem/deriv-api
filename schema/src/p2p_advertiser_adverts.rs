
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_advertiser_adverts/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 
use serde_json::Value;
use std::collections::HashMap;


// Import shared types from the *same* crate

// It's a struct
/// List of the P2P advertiser adverts.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct P2pAdvertiserAdverts {
    /// Field 'list' mapped to Value due to complexity/potential issues.\n
    // Correct serde attribute construction - Use helper
    
    pub list: Value,
}

