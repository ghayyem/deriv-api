
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/new_account_maltainvest/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

// It's a struct
/// New `maltainvest` account details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NewAccountMaltainvest {
    /// Client ID of new `maltainvest` account\n
    // Correct serde attribute construction - Use helper
    
    pub client_id: String,
    /// Currency of an account\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub currency: Option<String>,
    /// Currency type against the currency\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub currency_type: Option<String>,
    /// Landing company full name\n
    // Correct serde attribute construction - Use helper
    
    pub landing_company: String,
    /// Landing company shortcode\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub landing_company_short: Option<String>,
    /// Landing company shortcode\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub landing_company_shortcode: Option<String>,
    /// OAuth token for client's login session\n
    // Correct serde attribute construction - Use helper
    
    pub oauth_token: String,
}

