
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/authorize/send.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;



// Import required types from the *same* crate
use crate::add_to_login_history::AddToLoginHistory;

/// Authorize current WebSocket session to act on behalf of the owner of a given token. Must precede requests that need to access client account, for example purchasing and selling contracts or viewing portfolio.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuthorizeRequest {
    /// [Optional] Send this when you use api tokens for authorization and want to track activity using `login_history` call.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub add_to_login_history: Option<AddToLoginHistory>,
    /// Authentication token. May be retrieved from https://www.binary.com/en/user/security/api_tokenws.html. Set to MULTI when using multiple tokens.\n
    // Correct serde attribute construction - Use helper
    
    pub authorize: String,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub passthrough: Option<Value>,
    /// [Optional] Used to map request to response.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub req_id: Option<i64>,
    /// Additional Authentication tokens of authorized user that may be used in this session. Upto 25 tokens.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub tokens: Option<Vec<String>>,
}

