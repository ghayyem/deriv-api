
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/cashier/receive.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;



// Import required types from the *same* crate

/// Cashier information for the specified type.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CashierResponse {
    /// Possible error codes are:\n/// - `ASK_TNC_APPROVAL`: API call `tnc_approval`\n/// - `ASK_AUTHENTICATE`\n/// - `ASK_UK_FUNDS_PROTECTION`: API call `tnc_approval`\n/// - `ASK_CURRENCY`: API call `set_account_currency`\n/// - `ASK_EMAIL_VERIFY`: API call `verify_email`\n/// - `ASK_FIX_DETAILS`: API call `set_settings`\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub cashier: Option<Value>,
    /// Echo of the request made.\n
    // Correct serde attribute construction - Use helper
    
    pub echo_req: Value,
    /// Action name of the request made.\n
    // Correct serde attribute construction - Use helper
    
    pub msg_type: String,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub req_id: Option<i64>,
}

