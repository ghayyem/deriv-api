
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/app_register/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

// It's a struct
/// The information of the created application.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AppRegister {
    /// Active.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub active: Option<i64>,
    /// Application ID.\n
    // Correct serde attribute construction - Use helper
    
    pub app_id: i64,
    /// Markup added to contract prices (as a percentage of contract payout).\n
    // Correct serde attribute construction - Use helper
    
    pub app_markup_percentage: f64,
    /// Application's App Store URL.\n
    // Correct serde attribute construction - Use helper
    
    pub appstore: String,
    /// Application's GitHub page (for open-source projects).\n
    // Correct serde attribute construction - Use helper
    
    pub github: String,
    /// Application's Google Play URL.\n
    // Correct serde attribute construction - Use helper
    
    pub googleplay: String,
    /// Application's homepage URL.\n
    // Correct serde attribute construction - Use helper
    
    pub homepage: String,
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
    
    pub verification_uri: String,
}

