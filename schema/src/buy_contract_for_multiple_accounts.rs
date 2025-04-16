
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/buy_contract_for_multiple_accounts/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 
use serde_json::Value;
use std::collections::HashMap;


// Import shared types from the *same* crate

// It's a struct
/// Receipt confirmation for the purchase
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BuyContractForMultipleAccounts {
    /// List of results containing transactions and/or errors for the bought contracts.\n
    // Correct serde attribute construction - Use helper
    
    pub result: Vec<Value>,
}

