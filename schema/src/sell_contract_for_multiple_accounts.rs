
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/sell_contract_for_multiple_accounts/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 
use serde_json::Value;
use std::collections::HashMap;


// Import shared types from the *same* crate

// It's a struct
/// Status information for each affected account.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SellContractForMultipleAccounts {
    /// The result of sell for multiple accounts request.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub result: Option<Vec<Value>>,
}

