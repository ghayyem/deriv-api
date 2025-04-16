
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/transfer_between_accounts/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::transfers_enum::TransfersEnum; 
use crate::market_type_enum::MarketTypeEnum; 
use crate::demo_account_enum::DemoAccountEnum; 
use crate::account_type_enum::AccountTypeEnum; 
use crate::sub_account_type_enum::SubAccountTypeEnum; 
use crate::account_category_enum::AccountCategoryEnum; 
use crate::product_enum::ProductEnum; 

// It's a struct
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AccountsItem {
    /// Category of the account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub account_category: Option<AccountCategoryEnum>,
    /// Type of the account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub account_type: Option<AccountTypeEnum>,
    /// Account balance.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub balance: Option<f64>,
    /// Default account currency.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub currency: Option<String>,
    /// 0 for real accounts; 1 for virtual/demo accounts.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub demo_account: Option<DemoAccountEnum>,
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
    pub market_type: Option<MarketTypeEnum>,
    /// The group of mt5 account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub mt5_group: Option<String>,
    /// Product name that Deriv offer\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub product: Option<ProductEnum>,
    /// The status of account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub status: Option<String>,
    /// Sub account type\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub sub_account_type: Option<SubAccountTypeEnum>,
    /// Type of transfers allowed between the account and the currently authorized account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub transfers: Option<TransfersEnum>,
}

