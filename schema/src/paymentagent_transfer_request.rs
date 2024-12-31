
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/paymentagent_transfer/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Payment Agent Transfer - this call is available only to accounts that are approved Payment Agents.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct PaymentagentTransferRequest {
    /// The amount to transfer.
    #[serde(rename = "amount")]
    pub amount: f64,
    /// Currency code.
    #[serde(rename = "currency")]
    pub currency: String,
    /// [Optional] Remarks about the transfer.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: String,
    /// [Optional] If set to `1`, just do validation.
    #[serde(rename = "dry_run", skip_serializing_if = "Option::is_none")]
    pub dry_run: DryRunEnum,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// Must be `1`
    #[serde(rename = "paymentagent_transfer")]
    pub paymentagent_transfer: PaymentagentTransferEnum,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// The loginid of the recipient account.
    #[serde(rename = "transfer_to")]
    pub transfer_to: String,
}




/// Must be `1`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentagentTransferEnum {
    Value1 = 1,
}

impl PaymentagentTransferEnum {
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
