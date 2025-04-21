
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/sell_contract_for_multiple_accounts/send.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;



// Import required types from the *same* crate

/// Sell contracts for multiple accounts simultaneously. Uses the shortcode response from `buy_contract_for_multiple_accounts` to identify the contract, and authorisation tokens to select which accounts to sell those contracts on. Note that only the accounts identified by the tokens will be affected. This will not sell the contract on the currently-authorised account unless you include the token for the current account.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SellContractForMultipleAccountsRequest {
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub loginid: Option<String>,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub passthrough: Option<Value>,
    /// Minimum price at which to sell the contract, or `0` for 'sell at market'.\n
    // Correct serde attribute construction - Use helper
    
    pub price: String,
    /// [Optional] Used to map request to response.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub req_id: Option<i64>,
    /// Must be `1`\n
    // Correct serde attribute construction - Use helper
    
    pub sell_contract_for_multiple_accounts: i64,
    /// An internal ID used to identify the contract which was originally bought. This is returned from the `buy` and `buy_contract_for_multiple_accounts` calls.\n
    // Correct serde attribute construction - Use helper
    
    pub shortcode: String,
    /// Authorisation tokens which select the accounts to sell use for the affected accounts.\n
    // Correct serde attribute construction - Use helper
    
    pub tokens: Vec<String>,
}

