
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



/// Currency code
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct LocalCurrenciesvalue {
    /// Number of fractional digits.
    #[serde(rename = "fractional_digits")]
    pub fractional_digits: i64,
}








