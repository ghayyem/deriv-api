
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/statement/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Retrieve a summary of account transactions, according to given search criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct StatementRequest {
    /// [Optional] To filter the statement according to the type of transaction.
    #[serde(rename = "action_type", skip_serializing_if = "Option::is_none")]
    pub action_type: ActionTypeEnum,
    /// [Optional] Start date (epoch)
    #[serde(rename = "date_from", skip_serializing_if = "Option::is_none")]
    pub date_from: i64,
    /// [Optional] End date (epoch)
    #[serde(rename = "date_to", skip_serializing_if = "Option::is_none")]
    pub date_to: i64,
    /// [Optional] If set to 1, will return full contracts description.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: DescriptionEnum,
    /// [Optional] Maximum number of transactions to receive.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: f64,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// [Optional] Number of transactions to skip.
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: i64,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// Must be `1`
    #[serde(rename = "statement")]
    pub statement: StatementEnum,
}




/// [Optional] To filter the statement according to the type of transaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionTypeEnum {
    Buy,
    Sell,
    Deposit,
    Withdrawal,
    Escrow,
    Adjustment,
    Virtual_Credit,
    Transfer,
}

impl ActionTypeEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Buy => "buy",
            Self::Sell => "sell",
            Self::Deposit => "deposit",
            Self::Withdrawal => "withdrawal",
            Self::Escrow => "escrow",
            Self::Adjustment => "adjustment",
            Self::Virtual_Credit => "virtual_credit",
            Self::Transfer => "transfer",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "buy" => Some(Self::Buy),
            "sell" => Some(Self::Sell),
            "deposit" => Some(Self::Deposit),
            "withdrawal" => Some(Self::Withdrawal),
            "escrow" => Some(Self::Escrow),
            "adjustment" => Some(Self::Adjustment),
            "virtual_credit" => Some(Self::Virtual_Credit),
            "transfer" => Some(Self::Transfer),
            _ => None,
        }
    }
}
/// Must be `1`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StatementEnum {
    Value1 = 1,
}

impl StatementEnum {
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
