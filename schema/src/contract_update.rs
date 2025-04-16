
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/contract_update/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::stop_loss::StopLoss; 
use crate::take_profit::TakeProfit; 

// It's a struct
/// Contains the update status of the request
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ContractUpdate {
    /// The target spot price where the contract will be closed automatically at the loss specified by the user.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub stop_loss: Option<StopLoss>,
    /// The target spot price where the contract will be closed automatically at the profit specified by the user.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub take_profit: Option<TakeProfit>,
}

