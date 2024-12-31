
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/mt5_new_account/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// This call creates new MT5 user, either demo or real money user.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Mt5NewAccountRequest {
    /// Account type. If set to 'financial', setting 'mt5_account_type' is also required.
    #[serde(rename = "account_type")]
    pub account_type: AccountTypeEnum,
    /// [Optional] The address of the user. The maximum length of this address field is 128 characters.
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: String,
    /// [Optional] User's city of residence.
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: String,
    /// [Optional] Name of the client's company. The maximum length of the company name is 64 characters.
    #[serde(rename = "company", skip_serializing_if = "Option::is_none")]
    pub company: String,
    /// [Optional] 2-letter country code (value received from `residence_list` call).
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: String,
    /// [Optional] MT5 account currency, the default value will be the qualified account currency.
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: String,
    /// [Optional] If set to 1, only validation is performed.
    #[serde(rename = "dry_run", skip_serializing_if = "Option::is_none")]
    pub dry_run: DryRunEnum,
    /// Email address
    #[serde(rename = "email")]
    pub email: String,
    /// [Optional] The investor password of the account. For validation (Accepts any printable ASCII character. Must be within 8-25 characters, and include numbers, lowercase and uppercase letters. Must not be the same as the user's email address).
    #[serde(rename = "investPassword", skip_serializing_if = "Option::is_none")]
    pub invest_password: String,
    /// Client leverage (from 1 to 1000).
    #[serde(rename = "leverage")]
    pub leverage: f64,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// The master password of the account. For validation (Accepts any printable ASCII character. Must be within 8-25 characters, and include numbers, lowercase and uppercase letters. Must not be the same as the user's email address). This field is required.
    #[serde(rename = "mainPassword")]
    pub main_password: String,
    /// [Optional] Indicates whether the user would like to migrate his account to other jurisdiction.
    #[serde(rename = "migrate", skip_serializing_if = "Option::is_none")]
    pub migrate: bool,
    /// [Optional] To choose whether account is conventional or swap_free. Unavailable for financial_stp MT5_account_type
    #[serde(rename = "mt5_account_category", skip_serializing_if = "Option::is_none")]
    pub mt_5_account_category: Mt5AccountCategoryEnum,
    /// [Optional] Financial: Variable spreads, High leverage. Financial STP: Variable spreads, Medium Leverage, more products. If 'account_type' set to 'financial', setting 'mt5_account_type' is also required.
    #[serde(rename = "mt5_account_type", skip_serializing_if = "Option::is_none")]
    pub mt_5_account_type: Mt5AccountTypeEnum,
    /// Must be `1`
    #[serde(rename = "mt5_new_account")]
    pub mt_5_new_account: Mt5NewAccountEnum,
    /// Client's name. The maximum length here is 101 characters.
    #[serde(rename = "name")]
    pub name: String,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] User's phone number.
    #[serde(rename = "phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<Value>,
    /// [Optional] The user's phone password.
    #[serde(rename = "phonePassword", skip_serializing_if = "Option::is_none")]
    pub phone_password: String,
    /// Product name that Deriv offer
    #[serde(rename = "product", skip_serializing_if = "Option::is_none")]
    pub product: ProductEnum,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// [Optional] Trade server.
    #[serde(rename = "server", skip_serializing_if = "Option::is_none")]
    pub server: Option<ServerEnum>,
    /// [Optional] User's state (region) of residence.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: String,
    /// [Optional] Indicate the additional risk management for each account
    #[serde(rename = "sub_account_category", skip_serializing_if = "Option::is_none")]
    pub sub_account_category: SubAccountCategoryEnum,
    /// [Optional] Indicate the different offerings for mt5 account
    #[serde(rename = "sub_account_type", skip_serializing_if = "Option::is_none")]
    pub sub_account_type: SubAccountTypeEnum,
    /// [Optional] User's zip code.
    #[serde(rename = "zipCode", skip_serializing_if = "Option::is_none")]
    pub zip_code: String,
}




/// [Optional] To choose whether account is conventional or swap_free. Unavailable for financial_stp MT5_account_type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mt5AccountCategoryEnum {
    Conventional,
    Swap_Free,
    Gold,
}

impl Mt5AccountCategoryEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Conventional => "conventional",
            Self::Swap_Free => "swap_free",
            Self::Gold => "gold",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "conventional" => Some(Self::Conventional),
            "swap_free" => Some(Self::Swap_Free),
            "gold" => Some(Self::Gold),
            _ => None,
        }
    }
}
/// [Optional] Financial: Variable spreads, High leverage. Financial STP: Variable spreads, Medium Leverage, more products. If 'account_type' set to 'financial', setting 'mt5_account_type' is also required.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mt5AccountTypeEnum {
    Financial,
    Financial_Stp,
    Gold,
}

impl Mt5AccountTypeEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Financial => "financial",
            Self::Financial_Stp => "financial_stp",
            Self::Gold => "gold",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "financial" => Some(Self::Financial),
            "financial_stp" => Some(Self::Financial_Stp),
            "gold" => Some(Self::Gold),
            _ => None,
        }
    }
}
/// Must be `1`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mt5NewAccountEnum {
    Value1 = 1,
}

impl Mt5NewAccountEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Value1 => "1",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "1" => Some(Self::Value1),
            _ => None,
        }
    }
}
/// [Optional] Trade server.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServerEnum {
    P01_Ts01,
    P01_Ts02,
    P01_Ts03,
    P01_Ts04,
    P02_Ts02,
    P03_Ts01,
    P03_Ts02,
}

impl ServerEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::P01_Ts01 => "p01_ts01",
            Self::P01_Ts02 => "p01_ts02",
            Self::P01_Ts03 => "p01_ts03",
            Self::P01_Ts04 => "p01_ts04",
            Self::P02_Ts02 => "p02_ts02",
            Self::P03_Ts01 => "p03_ts01",
            Self::P03_Ts02 => "p03_ts02",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "p01_ts01" => Some(Self::P01_Ts01),
            "p01_ts02" => Some(Self::P01_Ts02),
            "p01_ts03" => Some(Self::P01_Ts03),
            "p01_ts04" => Some(Self::P01_Ts04),
            "p02_ts02" => Some(Self::P02_Ts02),
            "p03_ts01" => Some(Self::P03_Ts01),
            "p03_ts02" => Some(Self::P03_Ts02),
            _ => None,
        }
    }
}
