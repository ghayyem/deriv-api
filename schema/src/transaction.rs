
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/transaction/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::action_enum::ActionEnum; 

// It's a struct
/// Realtime stream of user transaction updates.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Transaction {
    /// The transaction type.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub action: Option<ActionEnum>,
    /// It is the amount of transaction performed.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub amount: Option<f64>,
    /// Balance amount\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub balance: Option<f64>,
    /// Barrier of the contract. Only applicable to single barrier contracts. Could be undefined if a contract does not have a barrier.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub barrier: Option<f64>,
    /// It is the contract ID.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub contract_id: Option<i64>,
    /// Transaction currency\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub currency: Option<String>,
    /// Epoch value of the expiry time of the contract. Please note that in case of buy transaction this is approximate value not exact one.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub date_expiry: Option<i64>,
    /// Display name of symbol\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub display_name: Option<String>,
    /// The high barrier of a contract. Only applicable to double barrier contracts.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub high_barrier: Option<f64>,
    /// A per-connection unique identifier. Can be passed to the `forget` API call to unsubscribe.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub id: Option<String>,
    /// Description of contract purchased\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub longcode: Option<String>,
    /// The low barrier of a contract. Only applicable to double barrier contracts.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub low_barrier: Option<String>,
    /// Time at which contract was purchased, present only for sell transaction\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub purchase_time: Option<i64>,
    /// The pip-sized target spot price where the contract will be closed automatically at the loss specified by the user.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub stop_loss: Option<f64>,
    /// The pip-sized target spot price where the contract will be closed automatically when the value of the contract is close to zero. This is set by the us.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub stop_out: Option<f64>,
    /// Symbol code\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub symbol: Option<String>,
    /// The pip-sized target spot price where the contract will be closed automatically at the profit specified by the user.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub take_profit: Option<f64>,
    /// It is the transaction ID. Every contract (buy or sell) or payment has a unique ID.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub transaction_id: Option<i64>,
    /// Time at which transaction was performed: for buy it is purchase time, for sell it is sell time.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub transaction_time: Option<i64>,
}

