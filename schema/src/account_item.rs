
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/transfer_between_accounts/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 
use serde_json::Value;



// Import shared types from the *same* crate
use crate::account_type::AccountType; 
use crate::product::Product; 
use crate::sub_account_type::SubAccountType; 
use crate::transfers::Transfers; 
use crate::market_type::MarketType; 
use crate::demo_account::DemoAccount; 
use crate::account_category::AccountCategory; 

// It's a struct
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AccountItem {
    /// Category of the account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub account_category: Option<AccountCategory>,
    /// Type of the account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub account_type: Option<AccountType>,
    /// Account balance.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub balance: Option<String>,
    /// Default account currency.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub currency: Option<String>,
    /// 0 for real accounts; 1 for virtual/demo accounts.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub demo_account: Option<DemoAccount>,
    /// Landing company shortcode of the Trading account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub landing_company_short: Option<String>,
    /// Account identifier used for system transfers.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub loginid: Option<String>,
    /// Market type of account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub market_type: Option<MarketType>,
    /// The group of mt5 account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub mt5_group: Option<String>,
    /// Product name that Deriv offer\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub product: Option<Product>,
    /// The status of account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub status: Option<Value>,
    /// Sub account type\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub sub_account_type: Option<SubAccountType>,
    /// Type of transfers allowed between the account and the currently authorized account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub transfers: Option<Transfers>,
}

