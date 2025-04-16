
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/copytrading_list/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 
use serde_json::Value;
use std::collections::HashMap;


// Import shared types from the *same* crate
use crate::copiers_item::CopiersItem; 

// It's a struct
/// The trading information of copiers or traders.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CopytradingList {
    /// List of users who are currently copy trading the authenticated user\n
    // Correct serde attribute construction - Use helper
    
    pub copiers: Vec<CopiersItem>,
    /// Field 'traders' mapped to Value due to complexity/potential issues.\n
    // Correct serde attribute construction - Use helper
    
    pub traders: Value,
}

