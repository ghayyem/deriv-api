
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/new_account_virtual/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::type_enum::TypeEnum; 

// It's a struct
/// New virtual-money account details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NewAccountVirtual {
    /// Account balance\n
    // Correct serde attribute construction - Use helper
    
    pub balance: f64,
    /// ID of the new virtual-money account\n
    // Correct serde attribute construction - Use helper
    
    pub client_id: String,
    /// Account currency\n
    // Correct serde attribute construction - Use helper
    
    pub currency: String,
    /// Currency type against the currency\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub currency_type: Option<String>,
    /// Email of the new virtual-money account\n
    // Correct serde attribute construction - Use helper
    
    pub email: String,
    /// [Optional] One-time code for passwordless login, valid for 1 minute\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub login_code: Option<String>,
    /// Oauth token for the client's login session (so that the user may be logged in immediately)\n
    // Correct serde attribute construction - Use helper
    
    pub oauth_token: String,
    /// Refresh token to perform PTA, only for the first ever created account\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub refresh_token: Option<String>,
    /// Account type\n
    // Correct serde attribute construction - Use helper
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")] 
    pub r#type: Option<TypeEnum>,
}

