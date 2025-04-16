
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/mt5_withdrawal/send.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


// Import required types from the *same* crate
use crate::mt5_withdrawal_enum::Mt5WithdrawalEnum;

/// This call allows withdrawal from MT5 account to Binary account.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Mt5WithdrawalRequest {
    /// Amount to withdraw (in the currency of the MT5 account); min = $1 or an equivalent amount, max = $20000 or an equivalent amount.\n
    // Correct serde attribute construction - Use helper
    
    pub amount: f64,
    /// MT5 account login to withdraw money from\n
    // Correct serde attribute construction - Use helper
    
    pub from_mt5: String,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub loginid: Option<String>,
    /// Must be `1`\n
    // Correct serde attribute construction - Use helper
    
    pub mt5_withdrawal: Mt5WithdrawalEnum,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub passthrough: Option<Value>,
    /// [Optional] Used to map request to response.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub req_id: Option<i64>,
    /// Binary account loginid to transfer money to\n
    // Correct serde attribute construction - Use helper
    
    pub to_binary: String,
}

