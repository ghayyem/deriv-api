
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/paymentagent_withdraw/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Initiate a withdrawal to an approved Payment Agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct PaymentagentWithdrawRequest {
    /// The amount to withdraw to the payment agent.
    #[serde(rename = "amount")]
    pub amount: f64,
    /// The currency code.
    #[serde(rename = "currency")]
    pub currency: String,
    /// [Optional] Remarks about the withdraw. Only letters, numbers, space, period, comma, - ' are allowed.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: String,
    /// [Optional] If set to 1, just do validation.
    #[serde(rename = "dry_run", skip_serializing_if = "Option::is_none")]
    pub dry_run: DryRunEnum,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// The payment agent loginid received from the `paymentagent_list` call.
    #[serde(rename = "paymentagent_loginid")]
    pub paymentagent_loginid: String,
    /// Must be `1`
    #[serde(rename = "paymentagent_withdraw")]
    pub paymentagent_withdraw: PaymentagentWithdrawEnum,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// Email verification code (received from a `verify_email` call, which must be done first)
    #[serde(rename = "verification_code")]
    pub verification_code: String,
}




/// Must be `1`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentagentWithdrawEnum {
    Value1 = 1,
}

impl PaymentagentWithdrawEnum {
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
