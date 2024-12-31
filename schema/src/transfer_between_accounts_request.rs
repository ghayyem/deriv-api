
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/transfer_between_accounts/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// This call allows transfers between accounts held by a given user. Transfer funds between your fiat and cryptocurrency accounts (for a fee). Please note that account_from should be same as current authorized account.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct TransferBetweenAccountsRequest {
    /// [Optional] The loginid of the account to transfer funds from.
    #[serde(rename = "account_from", skip_serializing_if = "Option::is_none")]
    pub account_from: String,
    /// [Optional] The loginid of the account to transfer funds to.
    #[serde(rename = "account_to", skip_serializing_if = "Option::is_none")]
    pub account_to: String,
    /// [Optional] To control the list of accounts returned when `account_from` or `account_to` is not provided. `brief` (default value) means that accounts with `mt5` account_type will be excluded; it will run faster. `all` means that all accounts with any account_type (including `mt5`) will be returned.
    #[serde(rename = "accounts", skip_serializing_if = "Option::is_none")]
    pub accounts: AccountsEnum,
    /// [Optional] The amount to transfer.
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: f64,
    /// [Optional] Currency code.
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: String,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// If `account_from` or `account_to` is not provided, it just returns the available accounts.
    #[serde(rename = "transfer_between_accounts")]
    pub transfer_between_accounts: TransferBetweenAccountsEnum,
}




/// [Optional] To control the list of accounts returned when `account_from` or `account_to` is not provided. `brief` (default value) means that accounts with `mt5` account_type will be excluded; it will run faster. `all` means that all accounts with any account_type (including `mt5`) will be returned.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountsEnum {
    All,
    Brief,
}

impl AccountsEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Brief => "brief",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "all" => Some(Self::All),
            "brief" => Some(Self::Brief),
            _ => None,
        }
    }
}
/// If `account_from` or `account_to` is not provided, it just returns the available accounts.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransferBetweenAccountsEnum {
    Value1 = 1,
}

impl TransferBetweenAccountsEnum {
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
