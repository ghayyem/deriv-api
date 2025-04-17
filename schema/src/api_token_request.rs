
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/api_token/send.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


// Import required types from the *same* crate
use crate::api_token_enum::ApiTokenEnum;
use crate::valid_for_current_ip_only_enum::ValidForCurrentIpOnlyEnum;
use crate::new_token_scopes_item_enum::NewTokenScopesItemEnum;

/// This call manages API tokens
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ApiTokenRequest {
    /// Must be `1`\n
    // Correct serde attribute construction - Use helper
    
    pub api_token: ApiTokenEnum,
    /// [Optional] The token to remove.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub delete_token: Option<String>,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub loginid: Option<String>,
    /// [Optional] The name of the created token.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub new_token: Option<String>,
    /// [Optional] List of permission scopes to provide with the token.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub new_token_scopes: Option<Vec<NewTokenScopesItemEnum>>,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub passthrough: Option<Value>,
    /// [Optional] Used to map request to response.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub req_id: Option<i64>,
    /// [Optional] If you set this parameter during token creation, then the token created will only work for the IP address that was used to create the token\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub valid_for_current_ip_only: Option<ValidForCurrentIpOnlyEnum>,
}

