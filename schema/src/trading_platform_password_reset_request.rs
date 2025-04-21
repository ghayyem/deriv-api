
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/trading_platform_password_reset/send.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;



// Import required types from the *same* crate
use crate::platform::Platform;

/// Reset the password of a Trading Platform Account
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TradingPlatformPasswordResetRequest {
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub loginid: Option<String>,
    /// New password of the account. For validation (Accepts any printable ASCII character. DerivX: Must be within 8-25 characters, include numbers, lowercase, uppercase letters. Must not be the same as the user's email address). Accepts any printable ASCII character. MT5: Must be within 8-16 characters, include numbers, lowercase, uppercase letters and special characters. Must not be the same as the user's email address.\n
    // Correct serde attribute construction - Use helper
    
    pub new_password: String,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub passthrough: Option<Value>,
    /// Name of trading platform.\n
    // Correct serde attribute construction - Use helper
    
    pub platform: Platform,
    /// [Optional] Used to map request to response.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub req_id: Option<i64>,
    /// Must be `1`\n
    // Correct serde attribute construction - Use helper
    
    pub trading_platform_password_reset: i64,
    /// Email verification code (received from a `verify_email` call, which must be done first)\n
    // Correct serde attribute construction - Use helper
    
    pub verification_code: String,
}

