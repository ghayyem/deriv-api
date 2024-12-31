
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/mt5_new_account/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Create MT5 account Receive
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Mt5NewAccountResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// New MT5 account details
    #[serde(rename = "mt5_new_account", skip_serializing_if = "Option::is_none")]
    pub mt_5_new_account: Mt5NewAccount,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// New MT5 account details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Mt5NewAccount {
    /// Account type.
    #[serde(rename = "account_type", skip_serializing_if = "Option::is_none")]
    pub account_type: AccountTypeEnum,
    /// Agent Details.
    #[serde(rename = "agent", skip_serializing_if = "Option::is_none")]
    pub agent: Option<Value>,
    /// Account balance.
    #[serde(rename = "balance", skip_serializing_if = "Option::is_none")]
    pub balance: f64,
    /// MT5 account currency (`USD` or `EUR`) that depends on the MT5 company (`vanuatu`, `svg`, `malta`).
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: String,
    /// Account balance, formatted to appropriate decimal places.
    #[serde(rename = "display_balance", skip_serializing_if = "Option::is_none")]
    pub display_balance: String,
    /// Login ID of the user's new MT5 account. Login could have 2 types of prefixes: MTD, MTR. MTD - for demo accounts and MTR for real money accounts.
    #[serde(rename = "login", skip_serializing_if = "Option::is_none")]
    pub login: String,
    /// With default value of conventional, unavailable for `financial_stp` sub account type.
    #[serde(rename = "mt5_account_category", skip_serializing_if = "Option::is_none")]
    pub mt_5_account_category: Mt5AccountCategoryEnum,
    /// Sub account type for classic MT5 account.
    #[serde(rename = "mt5_account_type", skip_serializing_if = "Option::is_none")]
    pub mt_5_account_type: Mt5AccountTypeEnum,
    /// Product name that Deriv offer
    #[serde(rename = "product", skip_serializing_if = "Option::is_none")]
    pub product: ProductEnum,
    /// Indicate the different offerings for mt5 account.
    #[serde(rename = "sub_account_type", skip_serializing_if = "Option::is_none")]
    pub sub_account_type: SubAccountTypeEnum,
}






