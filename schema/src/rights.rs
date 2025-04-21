
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/mt5_login_list/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

// It's a struct
/// Rights assigned to the MT5 account.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Rights {
    /// User is allowed to connect via MT5 Web API\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub api: Option<bool>,
    /// This flag is obsolete and not used\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub api_deprecated: Option<bool>,
    /// User's certificate is confirmed\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub confirmed: Option<bool>,
    /// The User is allowed to connect\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub enabled: Option<bool>,
    /// User is not allowed to view reports\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub exclude_reports: Option<bool>,
    /// User is allowed to use Expert Advisors\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub expert: Option<bool>,
    /// For internal mt5 usage\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub investor: Option<bool>,
    /// User is allowed to use OTP\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub otp_enabled: Option<bool>,
    /// User is allowed to change password\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub password_change: Option<bool>,
    /// User has enabled push notifications\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub push: Option<bool>,
    /// Value for internal mt5 usage\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub readonly: Option<bool>,
    /// User is allowed to receive daily reports\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub reports: Option<bool>,
    /// User must change password during next connection\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub reset_pass: Option<bool>,
    /// VPS is enabled for user\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub sponsored: Option<bool>,
    /// User can view technical accounts in manager/admin terminal\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub technical: Option<bool>,
    /// Trading is disabled for user\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub trade_disabled: Option<bool>,
    /// User is allowed to use trailing stops\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub trailing: Option<bool>,
}

