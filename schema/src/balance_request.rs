
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/balance/send.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;



// Import required types from the *same* crate
use crate::subscribe::Subscribe;

/// Get user account balance
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BalanceRequest {
    /// [Optional] If set to `all`, return the balances of all accounts one by one; if set to `current`, return the balance of current account; if set as an account id, return the balance of that account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub account: Option<String>,
    /// Must be `1`\n
    // Correct serde attribute construction - Use helper
    
    pub balance: i64,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub loginid: Option<String>,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub passthrough: Option<Value>,
    /// [Optional] Used to map request to response.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub req_id: Option<i64>,
    /// [Optional] If set to 1, will send updates whenever the balance changes.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub subscribe: Option<Subscribe>,
}

