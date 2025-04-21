
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/new_partner_account/receive.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;



// Import required types from the *same* crate
use crate::new_partner_account::NewPartnerAccount;

/// Create real partner account - Receive
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NewPartnerAccountResponse {
    /// Echo of the request made.\n
    // Correct serde attribute construction - Use helper
    
    pub echo_req: Value,
    /// Action name of the request made.\n
    // Correct serde attribute construction - Use helper
    
    pub msg_type: String,
    /// New real partner account details\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub new_partner_account: Option<NewPartnerAccount>,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub req_id: Option<i64>,
}

