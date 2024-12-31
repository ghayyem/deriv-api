
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/authorize/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// A message containing account information for the holder of that token.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct AuthorizeResponse {
    /// Account information for the holder of the token.
    #[serde(rename = "authorize", skip_serializing_if = "Option::is_none")]
    pub authorize: Authorize,
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Account information for the holder of the token.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Authorize {
    /// List of accounts for current user. This is also available from the `account_list` call.
    #[serde(rename = "account_list", skip_serializing_if = "Option::is_none")]
    pub account_list: Vec<AccountListitem>,
    /// Cash balance of the account.
    #[serde(rename = "balance", skip_serializing_if = "Option::is_none")]
    pub balance: f64,
    /// 2-letter country code (ISO standard).
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: String,
    /// Currency of the account.
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: String,
    /// User email.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: String,
    /// User's full name. Will be empty for virtual accounts.
    #[serde(rename = "fullname", skip_serializing_if = "Option::is_none")]
    pub fullname: String,
    /// Boolean value: 1 or 0, indicating whether the account is a virtual-money account.
    #[serde(rename = "is_virtual", skip_serializing_if = "Option::is_none")]
    pub is_virtual: IsVirtualEnum,
    /// Landing company name the account belongs to.
    #[serde(rename = "landing_company_fullname", skip_serializing_if = "Option::is_none")]
    pub landing_company_fullname: String,
    /// Landing company shortcode the account belongs to.
    #[serde(rename = "landing_company_name", skip_serializing_if = "Option::is_none")]
    pub landing_company_name: String,
    /// Details of the list of Trading accounts linked to the Wallet account.
    #[serde(rename = "linked_to", skip_serializing_if = "Option::is_none")]
    pub linked_to: Vec<LinkedToitem>,
    /// Currencies in client's residence country
    #[serde(rename = "local_currencies", skip_serializing_if = "Option::is_none", flatten)]
    pub local_currencies: HashMap<String, LocalCurrenciesvalue>,
    /// The account ID that the token was issued for.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// User's preferred language, ISO standard code of language
    #[serde(rename = "preferred_language", skip_serializing_if = "Option::is_none")]
    pub preferred_language: Option<Value>,
    /// Scopes available to the token.
    #[serde(rename = "scopes", skip_serializing_if = "Option::is_none")]
    pub scopes: Vec<String>,
    /// List of landing company shortcodes the account can upgrade to.
    #[serde(rename = "upgradeable_landing_companies", skip_serializing_if = "Option::is_none")]
    pub upgradeable_landing_companies: Vec<Value>,
    /// The internal user ID for this account.
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: i64,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;

use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct AccountListitem {
    /// Account category.
    #[serde(rename = "account_category", skip_serializing_if = "Option::is_none")]
    pub account_category: AccountCategoryEnum,
    /// Account type.
    #[serde(rename = "account_type", skip_serializing_if = "Option::is_none")]
    pub account_type: String,
    /// 2 letter broker code.
    #[serde(rename = "broker", skip_serializing_if = "Option::is_none")]
    pub broker: String,
    /// Creation time of the account as epoch.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: i64,
    /// Currency of specified account.
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: String,
    /// Currency type for the corresponding currency.
    #[serde(rename = "currency_type", skip_serializing_if = "Option::is_none")]
    pub currency_type: String,
    /// Epoch of date till client has excluded him/herself from the website, only present if client is self excluded.
    #[serde(rename = "excluded_until", skip_serializing_if = "Option::is_none")]
    pub excluded_until: i64,
    /// Boolean value: 1 or 0, indicating whether the account is marked as disabled or not.
    #[serde(rename = "is_disabled", skip_serializing_if = "Option::is_none")]
    pub is_disabled: IsDisabledEnum,
    /// Boolean value: 1 or 0, indicating whether the account is a virtual-money account.
    #[serde(rename = "is_virtual", skip_serializing_if = "Option::is_none")]
    pub is_virtual: IsVirtualEnum,
    /// Landing company shortcode the account belongs to.
    #[serde(rename = "landing_company_name", skip_serializing_if = "Option::is_none")]
    pub landing_company_name: String,
    /// Details of the list of Trading accounts linked to the Wallet account.
    #[serde(rename = "linked_to", skip_serializing_if = "Option::is_none")]
    pub linked_to: Vec<LinkedToitem>,
    /// The account ID of specified account.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct LinkedToitem {
    /// Account ID.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// Account platform name.
    #[serde(rename = "platform", skip_serializing_if = "Option::is_none")]
    pub platform: PlatformEnum,
}






/// Boolean value: 1 or 0, indicating whether the account is marked as disabled or not.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IsDisabledEnum {
    Value1 = 1,
    Value0,
}

impl IsDisabledEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Value1 => "1",
            Self::Value0 => "0",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "1" => Some(Self::Value1),
            "0" => Some(Self::Value0),
            _ => None,
        }
    }
}
/// Boolean value: 1 or 0, indicating whether the account is a virtual-money account.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IsVirtualEnum {
    Value1 = 1,
    Value0,
}

impl IsVirtualEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Value1 => "1",
            Self::Value0 => "0",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "1" => Some(Self::Value1),
            "0" => Some(Self::Value0),
            _ => None,
        }
    }
}


// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Currency code
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct LocalCurrenciesvalue {
    /// Number of fractional digits.
    #[serde(rename = "fractional_digits")]
    pub fractional_digits: i64,
}








