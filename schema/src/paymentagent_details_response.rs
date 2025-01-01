
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/paymentagent_details/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Gets client's payment agent details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct PaymentagentDetailsResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// The payment agent details.
    #[serde(rename = "paymentagent_details", skip_serializing_if = "Option::is_none")]
    pub paymentagent_details: PaymentagentDetails,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// The payment agent details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct PaymentagentDetails {
    /// Client's My Affiliate id, if exists.
    #[serde(rename = "affiliate_id", skip_serializing_if = "Option::is_none")]
    pub affiliate_id: Option<Value>,
    /// If 1, the client may apply using paymentagent_create.
    #[serde(rename = "can_apply")]
    pub can_apply: CanApplyEnum,
    /// Indicates client's agreement with the Code of Conduct document.
    #[serde(rename = "code_of_conduct_approval", skip_serializing_if = "Option::is_none")]
    pub code_of_conduct_approval: CodeOfConductApprovalEnum,
    /// Commission (%) the agent want to take on deposits
    #[serde(rename = "commission_deposit", skip_serializing_if = "Option::is_none")]
    pub commission_deposit: f64,
    /// Commission (%) the agent want to take on withdrawals
    #[serde(rename = "commission_withdrawal", skip_serializing_if = "Option::is_none")]
    pub commission_withdrawal: f64,
    /// Currency supported by the payment agent. It's usually the same as agent's Deriv account currency.
    #[serde(rename = "currency_code", skip_serializing_if = "Option::is_none")]
    pub currency_code: String,
    /// Contains a list of error codes that would prevent a successful payment agent application.
    #[serde(rename = "eligibilty_validation", skip_serializing_if = "Option::is_none")]
    pub eligibilty_validation: Vec<String>,
    /// Payment agent's email address.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: String,
    /// Information about payment agent and their proposed service.
    #[serde(rename = "information", skip_serializing_if = "Option::is_none")]
    pub information: String,
    /// Maximum amount allowed for withdrawals
    #[serde(rename = "max_withdrawal", skip_serializing_if = "Option::is_none")]
    pub max_withdrawal: f64,
    /// Minimum amount allowed for withdrawals
    #[serde(rename = "min_withdrawal", skip_serializing_if = "Option::is_none")]
    pub min_withdrawal: f64,
    /// Indicates if the payment agent was recently approved with no transactions yet.
    #[serde(rename = "newly_authorized", skip_serializing_if = "Option::is_none")]
    pub newly_authorized: NewlyAuthorizedEnum,
    /// The name with which the payment agent is going to be identified.
    #[serde(rename = "payment_agent_name", skip_serializing_if = "Option::is_none")]
    pub payment_agent_name: String,
    /// Payment agent's phone number(s) with country code.
    #[serde(rename = "phone_numbers", skip_serializing_if = "Option::is_none")]
    pub phone_numbers: Vec<PhoneNumbersitem>,
    /// Indicates the status of the Payment Agent.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Value,
    /// A list of supported payment methods.
    #[serde(rename = "supported_payment_methods", skip_serializing_if = "Option::is_none")]
    pub supported_payment_methods: Vec<SupportedPaymentMethodsitem>,
    /// Client's target country.
    #[serde(rename = "target_country", skip_serializing_if = "Option::is_none")]
    pub target_country: String,
    /// The URL(s) of payment agent's website(s).
    #[serde(rename = "urls", skip_serializing_if = "Option::is_none")]
    pub urls: Vec<Urlsitem>,
}




/// If 1, the client may apply using paymentagent_create.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CanApplyEnum {
    Value0,
    Value1 = 1,
}

impl CanApplyEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Value0 => "0",
            Self::Value1 => "1",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "0" => Some(Self::Value0),
            "1" => Some(Self::Value1),
            _ => None,
        }
    }
}
/// Indicates if the payment agent was recently approved with no transactions yet.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NewlyAuthorizedEnum {
    Value0,
    Value1 = 1,
}

impl NewlyAuthorizedEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Value0 => "0",
            Self::Value1 => "1",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "0" => Some(Self::Value0),
            "1" => Some(Self::Value1),
            _ => None,
        }
    }
}


