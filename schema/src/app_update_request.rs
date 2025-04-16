
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/app_update/send.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


// Import required types from the *same* crate
use crate::scopes_item_enum::ScopesItemEnum;

/// Update a new OAuth application
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AppUpdateRequest {
    /// [Optional] Markup to be added to contract prices (as a percentage of contract payout). Max markup: 3%.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub app_markup_percentage: Option<f64>,
    /// Application app_id.\n
    // Correct serde attribute construction - Use helper
    
    pub app_update: i64,
    /// [Optional] Application's App Store URL (if applicable).\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub appstore: Option<String>,
    /// [Optional] Application's GitHub page (for open-source projects).\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub github: Option<String>,
    /// [Optional] Application's Google Play URL (if applicable).\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub googleplay: Option<String>,
    /// [Optional] Application's homepage URL.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub homepage: Option<String>,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub loginid: Option<String>,
    /// Application name.\n
    // Correct serde attribute construction - Use helper
    
    pub name: String,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub passthrough: Option<Value>,
    /// [Optional] The URL to redirect to after a successful login. Required if charging markup percentage.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub redirect_uri: Option<String>,
    /// [Optional] Used to map request to response.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub req_id: Option<i64>,
    /// Change scopes will revoke all user's grants and log them out.\n
    // Correct serde attribute construction - Use helper
    
    pub scopes: Vec<ScopesItemEnum>,
    /// [Optional] Used when `verify_email` called. If available, a URL containing the verification token will send to the client's email, otherwise only the token will be sent.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub verification_uri: Option<String>,
}

