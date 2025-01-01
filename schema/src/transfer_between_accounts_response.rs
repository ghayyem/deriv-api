
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/transfer_between_accounts/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// The result of transfer order.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct TransferBetweenAccountsResponse {
    /// The available accounts to transfer, or the accounts affected by a successful transfer.
    #[serde(rename = "accounts", skip_serializing_if = "Option::is_none")]
    pub accounts: Vec<Accountsitem>,
    /// The account to client full name
    #[serde(rename = "client_to_full_name", skip_serializing_if = "Option::is_none")]
    pub client_to_full_name: String,
    /// The account to client loginid
    #[serde(rename = "client_to_loginid", skip_serializing_if = "Option::is_none")]
    pub client_to_loginid: String,
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// Reference ID of transfer performed
    #[serde(rename = "transaction_id", skip_serializing_if = "Option::is_none")]
    pub transaction_id: i64,
    /// If set to 1, transfer succeeded.
    #[serde(rename = "transfer_between_accounts", skip_serializing_if = "Option::is_none")]
    pub transfer_between_accounts: TransferBetweenAccountsEnum,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Accountsitem {
    /// Category of the account.
    #[serde(rename = "account_category", skip_serializing_if = "Option::is_none")]
    pub account_category: AccountCategoryEnum,
    /// Type of the account.
    #[serde(rename = "account_type", skip_serializing_if = "Option::is_none")]
    pub account_type: AccountTypeEnum,
    /// Account balance.
    #[serde(rename = "balance", skip_serializing_if = "Option::is_none")]
    pub balance: String,
    /// Default account currency.
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: String,
    /// 0 for real accounts; 1 for virtual/demo accounts.
    #[serde(rename = "demo_account", skip_serializing_if = "Option::is_none")]
    pub demo_account: DemoAccountEnum,
    /// Landing company shortcode of the Trading account.
    #[serde(rename = "landing_company_short", skip_serializing_if = "Option::is_none")]
    pub landing_company_short: String,
    /// Account identifier used for system transfers.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// Market type of account.
    #[serde(rename = "market_type", skip_serializing_if = "Option::is_none")]
    pub market_type: MarketTypeEnum,
    /// The group of mt5 account.
    #[serde(rename = "mt5_group", skip_serializing_if = "Option::is_none")]
    pub mt_5_group: String,
    /// Product name that Deriv offer
    #[serde(rename = "product", skip_serializing_if = "Option::is_none")]
    pub product: ProductEnum,
    /// The status of account.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Value>,
    /// Sub account type
    #[serde(rename = "sub_account_type", skip_serializing_if = "Option::is_none")]
    pub sub_account_type: SubAccountTypeEnum,
    /// Type of transfers allowed between the account and the currently authorized account.
    #[serde(rename = "transfers", skip_serializing_if = "Option::is_none")]
    pub transfers: TransfersEnum,
}




/// Type of transfers allowed between the account and the currently authorized account.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransfersEnum {
    All,
    Deposit,
    None,
    Withdrawal,
}

impl TransfersEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Deposit => "deposit",
            Self::None => "none",
            Self::Withdrawal => "withdrawal",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "all" => Some(Self::All),
            "deposit" => Some(Self::Deposit),
            "none" => Some(Self::None),
            "withdrawal" => Some(Self::Withdrawal),
            _ => None,
        }
    }
}


