
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/sell_contract_for_multiple_accounts/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Sell contracts for multiple accounts simultaneously. Uses the shortcode response from `buy_contract_for_multiple_accounts` to identify the contract, and authorisation tokens to select which accounts to sell those contracts on. Note that only the accounts identified by the tokens will be affected. This will not sell the contract on the currently-authorised account unless you include the token for the current account.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct SellContractForMultipleAccountsRequest {
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// Minimum price at which to sell the contract, or `0` for 'sell at market'.
    #[serde(rename = "price")]
    pub price: f64,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// Must be `1`
    #[serde(rename = "sell_contract_for_multiple_accounts")]
    pub sell_contract_for_multiple_accounts: SellContractForMultipleAccountsEnum,
    /// An internal ID used to identify the contract which was originally bought. This is returned from the `buy` and `buy_contract_for_multiple_accounts` calls.
    #[serde(rename = "shortcode")]
    pub shortcode: String,
    /// Authorisation tokens which select the accounts to sell use for the affected accounts.
    #[serde(rename = "tokens")]
    pub tokens: Vec<String>,
}




/// Must be `1`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SellContractForMultipleAccountsEnum {
    Value1 = 1,
}

impl SellContractForMultipleAccountsEnum {
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
