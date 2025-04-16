
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/app_register/send.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


// Import required types from the *same* crate
use crate::app_register_enum::AppRegisterEnum;
use crate::scopes_item_enum::ScopesItemEnum;

/// Register a new OAuth application
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AppRegisterRequest {
    /// [Optional] Markup to be added to contract prices (as a percentage of contract payout). Max markup: 3%.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub app_markup_percentage: Option<f64>,
    /// Must be `1`\n
    // Correct serde attribute construction - Use helper
    
    pub app_register: AppRegisterEnum,
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
    /// [Optional] The URL to redirect to after a successful login. Required if charging markup percentage\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub redirect_uri: Option<String>,
    /// [Optional] Used to map request to response.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub req_id: Option<i64>,
    /// List of permission scopes to grant the application.\n
    // Correct serde attribute construction - Use helper
    
    pub scopes: Vec<ScopesItemEnum>,
    /// [Optional] Used when `verify_email` called. If available, a URL containing the verification token will be sent to the client's email, otherwise only the token will be sent.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub verification_uri: Option<String>,
}

