
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/balance/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 
use serde_json::Value;
use std::collections::HashMap;


// Import shared types from the *same* crate
use crate::total::Total; 
use crate::accounts_value::AccountsValue; 

// It's a struct
/// Current balance of one or more accounts.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Balance {
    /// List of active accounts.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub accounts: Option<HashMap<String, AccountsValue>>,
    /// Balance of current account.\n
    // Correct serde attribute construction - Use helper
    
    pub balance: f64,
    /// Currency of current account.\n
    // Correct serde attribute construction - Use helper
    
    pub currency: String,
    /// A per-connection unique identifier. Can be passed to the `forget` API call to unsubscribe.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub id: Option<String>,
    /// Client loginid.\n
    // Correct serde attribute construction - Use helper
    
    pub loginid: String,
    /// Summary totals of accounts by type.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub total: Option<Total>,
}

