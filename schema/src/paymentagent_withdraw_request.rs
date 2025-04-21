
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/paymentagent_withdraw/send.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;



// Import required types from the *same* crate
use crate::dry_run::DryRun;

/// Initiate a withdrawal to an approved Payment Agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PaymentagentWithdrawRequest {
    /// The amount to withdraw to the payment agent.\n
    // Correct serde attribute construction - Use helper
    
    pub amount: String,
    /// The currency code.\n
    // Correct serde attribute construction - Use helper
    
    pub currency: String,
    /// [Optional] Remarks about the withdraw. Only letters, numbers, space, period, comma, - ' are allowed.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub description: Option<String>,
    /// [Optional] If set to 1, just do validation.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub dry_run: Option<DryRun>,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub loginid: Option<String>,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub passthrough: Option<Value>,
    /// The payment agent loginid received from the `paymentagent_list` call.\n
    // Correct serde attribute construction - Use helper
    
    pub paymentagent_loginid: String,
    /// Must be `1`\n
    // Correct serde attribute construction - Use helper
    
    pub paymentagent_withdraw: i64,
    /// [Optional] Used to map request to response.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub req_id: Option<i64>,
    /// Email verification code (received from a `verify_email` call, which must be done first)\n
    // Correct serde attribute construction - Use helper
    
    pub verification_code: String,
}

