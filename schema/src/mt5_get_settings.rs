
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/mt5_get_settings/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::account_type_enum::AccountTypeEnum; 
use crate::sub_account_type_enum::SubAccountTypeEnum; 
use crate::landing_company_short_enum::LandingCompanyShortEnum; 
use crate::server_info::ServerInfo; 
use crate::sub_account_category_enum::SubAccountCategoryEnum; 
use crate::market_type_enum::MarketTypeEnum; 

// It's a struct
/// MT5 user account details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Mt5GetSettings {
    /// Account type.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub account_type: Option<AccountTypeEnum>,
    /// The address of the user. The maximum length of the address is 128 characters.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub address: Option<String>,
    /// Balance of the Trading account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub balance: Option<f64>,
    /// User's city of residence.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub city: Option<String>,
    /// Name of the client's company. The maximum length of the company name is 64 characters.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub company: Option<String>,
    /// 2-letter country code.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub country: Option<String>,
    /// MT5 account currency (`USD` or `EUR`) that depends on the MT5 company (`vanuatu`, `svg`, `malta`).\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub currency: Option<String>,
    /// Account balance, formatted to appropriate decimal places.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub display_balance: Option<String>,
    /// Email address.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub email: Option<String>,
    /// The group where account belongs to.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub group: Option<String>,
    /// Landing company shortcode of the MT5 account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub landing_company_short: Option<LandingCompanyShortEnum>,
    /// Client leverage (from 1 to 1000).\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub leverage: Option<f64>,
    /// Login ID of the user's MT5 account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub login: Option<String>,
    /// Market type\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub market_type: Option<MarketTypeEnum>,
    /// Client's name. The maximum length of a client's symbol name is 128 characters.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub name: Option<String>,
    /// User's phone number.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub phone: Option<String>,
    /// The user's phone password.\n
    // Correct serde attribute construction - Use helper
    #[serde(rename = "phonePassword", skip_serializing_if = "Option::is_none")] 
    pub phone_password: Option<String>,
    /// Trade server name of the MT5 account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub server: Option<String>,
    /// Trade server information.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub server_info: Option<ServerInfo>,
    /// User's state (region) of residence.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub state: Option<String>,
    /// Sub account category.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub sub_account_category: Option<SubAccountCategoryEnum>,
    /// Sub account type\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub sub_account_type: Option<SubAccountTypeEnum>,
    /// User's zip code.\n
    // Correct serde attribute construction - Use helper
    #[serde(rename = "zipCode", skip_serializing_if = "Option::is_none")] 
    pub zip_code: Option<String>,
}

