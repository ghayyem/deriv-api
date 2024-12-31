
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/mt5_withdrawal/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// This call allows withdrawal from MT5 account to Binary account.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Mt5WithdrawalRequest {
    /// Amount to withdraw (in the currency of the MT5 account); min = $1 or an equivalent amount, max = $20000 or an equivalent amount.
    #[serde(rename = "amount")]
    pub amount: f64,
    /// MT5 account login to withdraw money from
    #[serde(rename = "from_mt5")]
    pub from_mt_5: String,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// Must be `1`
    #[serde(rename = "mt5_withdrawal")]
    pub mt_5_withdrawal: Mt5WithdrawalEnum,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// Binary account loginid to transfer money to
    #[serde(rename = "to_binary")]
    pub to_binary: String,
}




/// Must be `1`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mt5WithdrawalEnum {
    Value1 = 1,
}

impl Mt5WithdrawalEnum {
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
