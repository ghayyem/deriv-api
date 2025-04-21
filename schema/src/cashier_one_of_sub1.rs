
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/cashier/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 
use serde_json::Value;



// Import shared types from the *same* crate
use crate::action::Action; 
use crate::deposit::Deposit; 

// It's a struct
/// Response for type `api'.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CashierOneOfSub1 {
    /// Type of operation, which is requested.\n
    // Correct serde attribute construction - Use helper
    
    pub action: Action,
    /// [Optional] Result for `deposit` action.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub deposit: Option<Deposit>,
    /// [Optional] Result for `withdraw` action.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub withdraw: Option<Value>,
}

