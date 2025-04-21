
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/app_list/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 
use serde_json::Value;



// Import shared types from the *same* crate

// It's a struct
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AppListItem {
    /// Active.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub active: Option<i64>,
    /// Application ID.\n
    // Correct serde attribute construction - Use helper
    
    pub app_id: i64,
    /// Markup added to contract prices (as a percentage of contract payout).\n
    // Correct serde attribute construction - Use helper
    
    pub app_markup_percentage: String,
    /// Application's App Store URL.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub appstore: Option<Value>,
    /// Application's GitHub page. (for open-source projects)\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub github: Option<Value>,
    /// Application's Google Play URL.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub googleplay: Option<Value>,
    /// Application's homepage URL.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub homepage: Option<Value>,
    /// Application name.\n
    // Correct serde attribute construction - Use helper
    
    pub name: String,
    /// The URL to redirect to after a successful login.\n
    // Correct serde attribute construction - Use helper
    
    pub redirect_uri: String,
    /// Scope Details.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub scopes: Option<Vec<String>>,
    /// Used when `verify_email` called. If available, a URL containing the verification token will send to the client's email, otherwise only the token will be sent.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub verification_uri: Option<Value>,
}

