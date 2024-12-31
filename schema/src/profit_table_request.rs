
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/profit_table/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Retrieve a summary of account Profit Table, according to given search criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct ProfitTableRequest {
    /// Return only contracts of the specified types
    #[serde(rename = "contract_type", skip_serializing_if = "Option::is_none")]
    pub contract_type: Vec<ContractTypeitemEnum>,
    /// [Optional] Start date (epoch or YYYY-MM-DD)
    #[serde(rename = "date_from", skip_serializing_if = "Option::is_none")]
    pub date_from: String,
    /// [Optional] End date (epoch or YYYY-MM-DD)
    #[serde(rename = "date_to", skip_serializing_if = "Option::is_none")]
    pub date_to: String,
    /// [Optional] If set to 1, will return full contracts description.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: DescriptionEnum,
    /// [Optional] Apply upper limit to count of transactions received.
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
    /// Must be `1`
    #[serde(rename = "profit_table")]
    pub profit_table: ProfitTableEnum,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// [Optional] Sort direction.
    #[serde(rename = "sort", skip_serializing_if = "Option::is_none")]
    pub sort: SortEnum,
}




/// Must be `1`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfitTableEnum {
    Value1 = 1,
}

impl ProfitTableEnum {
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
