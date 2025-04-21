
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/buy_contract_for_multiple_accounts/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

// It's a struct
/// Error message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ResultItemAnyOfSub1 {
    /// An error code\n
    // Correct serde attribute construction - Use helper
    
    pub code: String,
    /// An error message localized according to the websocket\n
    // Correct serde attribute construction - Use helper
    
    pub message_to_client: String,
    /// The token designating the account\n
    // Correct serde attribute construction - Use helper
    
    pub token: String,
}

