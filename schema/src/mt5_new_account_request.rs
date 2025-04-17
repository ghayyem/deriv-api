
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/mt5_new_account/send.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


// Import required types from the *same* crate
use crate::account_type_enum::AccountTypeEnum;
use crate::sub_account_type_enum::SubAccountTypeEnum;
use crate::mt5_account_category_enum::Mt5AccountCategoryEnum;
use crate::server_enum::ServerEnum;
use crate::dry_run_enum::DryRunEnum;
use crate::product_enum::ProductEnum;
use crate::sub_account_category_enum::SubAccountCategoryEnum;
use crate::mt5_account_type_enum::Mt5AccountTypeEnum;

/// This call creates new MT5 user, either demo or real money user.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Mt5NewAccountRequest {
    /// Account type. If set to 'financial', setting 'mt5_account_type' is also required.\n
    // Correct serde attribute construction - Use helper
    
    pub account_type: AccountTypeEnum,
    /// [Optional] The address of the user. The maximum length of this address field is 128 characters.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub address: Option<String>,
    /// [Optional] User's city of residence.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub city: Option<String>,
    /// [Optional] Name of the client's company. The maximum length of the company name is 64 characters.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub company: Option<String>,
    /// [Optional] 2-letter country code (value received from `residence_list` call).\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub country: Option<f64>,
    /// [Optional] MT5 account currency, the default value will be the qualified account currency.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub currency: Option<f64>,
    /// [Optional] If set to 1, only validation is performed.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub dry_run: Option<DryRunEnum>,
    /// Email address\n
    // Correct serde attribute construction - Use helper
    
    pub email: String,
    /// [Optional] The investor password of the account. For validation (Accepts any printable ASCII character. Must be within 8-25 characters, and include numbers, lowercase and uppercase letters. Must not be the same as the user's email address).\n
    // Correct serde attribute construction - Use helper
    #[serde(rename = "investPassword", skip_serializing_if = "Option::is_none")] 
    pub invest_password: Option<String>,
    /// Client leverage (from 1 to 1000).\n
    // Correct serde attribute construction - Use helper
    
    pub leverage: f64,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub loginid: Option<String>,
    /// The master password of the account. For validation (Accepts any printable ASCII character. Must be within 8-25 characters, and include numbers, lowercase and uppercase letters. Must not be the same as the user's email address). This field is required.\n
    // Correct serde attribute construction - Use helper
    #[serde(rename = "mainPassword")] 
    pub main_password: String,
    /// [Optional] Indicates whether the user would like to migrate his account to other jurisdiction.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub migrate: Option<bool>,
    /// [Optional] To choose whether account is conventional or swap_free. Unavailable for financial_stp MT5_account_type\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub mt5_account_category: Option<Mt5AccountCategoryEnum>,
    /// [Optional] Financial: Variable spreads, High leverage. Financial STP: Variable spreads, Medium Leverage, more products. If 'account_type' set to 'financial', setting 'mt5_account_type' is also required.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub mt5_account_type: Option<Mt5AccountTypeEnum>,
    /// Field 'mt5_new_account' mapped to Value due to complexity/potential issues.\n
    // Correct serde attribute construction - Use helper
    
    pub mt5_new_account: Value,
    /// Client's name. The maximum length here is 101 characters.\n
    // Correct serde attribute construction - Use helper
    
    pub name: String,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub passthrough: Option<Value>,
    /// [Optional] User's phone number.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub phone: Option<String>,
    /// [Optional] The user's phone password.\n
    // Correct serde attribute construction - Use helper
    #[serde(rename = "phonePassword", skip_serializing_if = "Option::is_none")] 
    pub phone_password: Option<String>,
    /// Product name that Deriv offer\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub product: Option<ProductEnum>,
    /// [Optional] Used to map request to response.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub req_id: Option<i64>,
    /// [Optional] Trade server.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub server: Option<ServerEnum>,
    /// [Optional] User's state (region) of residence.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub state: Option<String>,
    /// [Optional] Indicate the additional risk management for each account\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub sub_account_category: Option<SubAccountCategoryEnum>,
    /// [Optional] Indicate the different offerings for mt5 account\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub sub_account_type: Option<SubAccountTypeEnum>,
    /// [Optional] User's zip code.\n
    // Correct serde attribute construction - Use helper
    #[serde(rename = "zipCode", skip_serializing_if = "Option::is_none")] 
    pub zip_code: Option<String>,
}

