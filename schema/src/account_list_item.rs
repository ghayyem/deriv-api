
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/authorize/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 


use chrono::{DateTime, Utc};

// Import shared types from the *same* crate
use crate::account_category_enum::AccountCategoryEnum; 
use crate::is_disabled_enum::IsDisabledEnum; 
use crate::is_virtual_enum::IsVirtualEnum; 
use crate::linked_to_item::LinkedToItem; 

// It's a struct
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AccountListItem {
    /// Account category.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub account_category: Option<AccountCategoryEnum>,
    /// Account type.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub account_type: Option<String>,
    /// 2 letter broker code.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub broker: Option<String>,
    /// Creation time of the account as epoch.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub created_at: Option<DateTime<Utc>>,
    /// Currency of specified account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub currency: Option<String>,
    /// Currency type for the corresponding currency.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub currency_type: Option<String>,
    /// Epoch of date till client has excluded him/herself from the website, only present if client is self excluded.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub excluded_until: Option<i64>,
    /// Boolean value: 1 or 0, indicating whether the account is marked as disabled or not.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub is_disabled: Option<IsDisabledEnum>,
    /// Boolean value: 1 or 0, indicating whether the account is a virtual-money account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub is_virtual: Option<IsVirtualEnum>,
    /// Landing company shortcode the account belongs to.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub landing_company_name: Option<String>,
    /// Details of the list of Trading accounts linked to the Wallet account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub linked_to: Option<Vec<LinkedToItem>>,
    /// The account ID of specified account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub loginid: Option<String>,
}

