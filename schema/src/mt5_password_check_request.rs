
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/mt5_password_check/send.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


// Import required types from the *same* crate
use crate::password_type_enum::PasswordTypeEnum;
use crate::mt5_password_check_enum::Mt5PasswordCheckEnum;

/// This call validates the main password for the MT5 user
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Mt5PasswordCheckRequest {
    /// MT5 user login\n
    // Correct serde attribute construction - Use helper
    
    pub login: String,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub loginid: Option<String>,
    /// Must be `1`\n
    // Correct serde attribute construction - Use helper
    
    pub mt5_password_check: Mt5PasswordCheckEnum,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub passthrough: Option<Value>,
    /// The password of the account.\n
    // Correct serde attribute construction - Use helper
    
    pub password: String,
    /// [Optional] Type of the password to check.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub password_type: Option<PasswordTypeEnum>,
    /// [Optional] Used to map request to response.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub req_id: Option<i64>,
}

