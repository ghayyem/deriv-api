
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/paymentagent_create/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Saves client's payment agent details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct PaymentagentCreateRequest {
    /// [Optional] Client's My Affiliate id, if exists.
    #[serde(rename = "affiliate_id", skip_serializing_if = "Option::is_none")]
    pub affiliate_id: String,
    /// Indicates client's agreement with the Code of Conduct.
    #[serde(rename = "code_of_conduct_approval")]
    pub code_of_conduct_approval: CodeOfConductApprovalEnum,
    /// Commission  (%) the agent wants to take on deposits
    #[serde(rename = "commission_deposit")]
    pub commission_deposit: f64,
    /// Commission  (%) the agent wants to take on withdrawals
    #[serde(rename = "commission_withdrawal")]
    pub commission_withdrawal: f64,
    /// Payment agent's email address.
    #[serde(rename = "email")]
    pub email: String,
    /// [Optional] Information about payment agent and their proposed service.
    #[serde(rename = "information")]
    pub information: String,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// The name with which the payment agent is going to be identified.
    #[serde(rename = "payment_agent_name")]
    pub payment_agent_name: String,
    /// Must be 1
    #[serde(rename = "paymentagent_create")]
    pub paymentagent_create: PaymentagentCreateEnum,
    /// Payment agent's phone number(s) with country code.
    #[serde(rename = "phone_numbers", skip_serializing_if = "Option::is_none")]
    pub phone_numbers: Vec<PhoneNumbersitem>,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// A list of supported payment methods.
    #[serde(rename = "supported_payment_methods")]
    pub supported_payment_methods: Vec<SupportedPaymentMethodsitem>,
    /// The URL(s) of payment agent's website(s).
    #[serde(rename = "urls")]
    pub urls: Vec<Urlsitem>,
}




/// Must be 1
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentagentCreateEnum {
    Value1 = 1,
}

impl PaymentagentCreateEnum {
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
