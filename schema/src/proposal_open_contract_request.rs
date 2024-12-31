
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/proposal_open_contract/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Get latest price (and other information) for a contract in the user's portfolio
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct ProposalOpenContractRequest {
    /// [Optional] Contract ID received from a `portfolio` request. If not set, you will receive stream of all open contracts.
    #[serde(rename = "contract_id", skip_serializing_if = "Option::is_none")]
    pub contract_id: i64,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// Must be `1`
    #[serde(rename = "proposal_open_contract")]
    pub proposal_open_contract: ProposalOpenContractEnum,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// [Optional] `1` to stream.
    #[serde(rename = "subscribe", skip_serializing_if = "Option::is_none")]
    pub subscribe: SubscribeEnum,
}




/// Must be `1`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProposalOpenContractEnum {
    Value1 = 1,
}

impl ProposalOpenContractEnum {
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
