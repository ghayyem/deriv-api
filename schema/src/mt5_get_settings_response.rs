
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/mt5_get_settings/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Get MT5 user settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Mt5GetSettingsResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// MT5 user account details
    #[serde(rename = "mt5_get_settings", skip_serializing_if = "Option::is_none")]
    pub mt_5_get_settings: Mt5GetSettings,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// MT5 user account details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Mt5GetSettings {
    /// Account type.
    #[serde(rename = "account_type", skip_serializing_if = "Option::is_none")]
    pub account_type: AccountTypeEnum,
    /// The address of the user. The maximum length of the address is 128 characters.
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: String,
    /// Balance of the Trading account.
    #[serde(rename = "balance", skip_serializing_if = "Option::is_none")]
    pub balance: f64,
    /// User's city of residence.
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: String,
    /// Name of the client's company. The maximum length of the company name is 64 characters.
    #[serde(rename = "company", skip_serializing_if = "Option::is_none")]
    pub company: String,
    /// 2-letter country code.
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: String,
    /// MT5 account currency (`USD` or `EUR`) that depends on the MT5 company (`vanuatu`, `svg`, `malta`).
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: String,
    /// Account balance, formatted to appropriate decimal places.
    #[serde(rename = "display_balance", skip_serializing_if = "Option::is_none")]
    pub display_balance: String,
    /// Email address.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: String,
    /// The group where account belongs to.
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: String,
    /// Landing company shortcode of the MT5 account.
    #[serde(rename = "landing_company_short", skip_serializing_if = "Option::is_none")]
    pub landing_company_short: LandingCompanyShortEnum,
    /// Client leverage (from 1 to 1000).
    #[serde(rename = "leverage", skip_serializing_if = "Option::is_none")]
    pub leverage: f64,
    /// Login ID of the user's MT5 account.
    #[serde(rename = "login", skip_serializing_if = "Option::is_none")]
    pub login: String,
    /// Market type
    #[serde(rename = "market_type", skip_serializing_if = "Option::is_none")]
    pub market_type: MarketTypeEnum,
    /// Client's name. The maximum length of a client's symbol name is 128 characters.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// User's phone number.
    #[serde(rename = "phone", skip_serializing_if = "Option::is_none")]
    pub phone: String,
    /// The user's phone password.
    #[serde(rename = "phonePassword", skip_serializing_if = "Option::is_none")]
    pub phone_password: String,
    /// Trade server name of the MT5 account.
    #[serde(rename = "server", skip_serializing_if = "Option::is_none")]
    pub server: String,
    /// Trade server information.
    #[serde(rename = "server_info", skip_serializing_if = "Option::is_none")]
    pub server_info: ServerInfo,
    /// User's state (region) of residence.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: String,
    /// Sub account category.
    #[serde(rename = "sub_account_category", skip_serializing_if = "Option::is_none")]
    pub sub_account_category: SubAccountCategoryEnum,
    /// Sub account type
    #[serde(rename = "sub_account_type", skip_serializing_if = "Option::is_none")]
    pub sub_account_type: SubAccountTypeEnum,
    /// User's zip code.
    #[serde(rename = "zipCode", skip_serializing_if = "Option::is_none")]
    pub zip_code: String,
}






