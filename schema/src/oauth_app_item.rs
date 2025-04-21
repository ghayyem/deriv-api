
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/oauth_apps/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 
use serde_json::Value;



// Import shared types from the *same* crate

// It's a struct
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OauthAppItem {
    /// Application ID.\n
    // Correct serde attribute construction - Use helper
    
    pub app_id: i64,
    /// Markup added to contract prices (as a percentage of contract payout)\n
    // Correct serde attribute construction - Use helper
    
    pub app_markup_percentage: String,
    /// The last date which the application has been used.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub last_used: Option<Value>,
    /// Application name\n
    // Correct serde attribute construction - Use helper
    
    pub name: String,
    /// Boolean value: 1 or 0, indicating 1 if app is an official app and 0 incase of unofficial app\n
    // Correct serde attribute construction - Use helper
    
    pub official: String,
    /// The list of permission scopes grant for each app.\n
    // Correct serde attribute construction - Use helper
    
    pub scopes: Vec<String>,
}

