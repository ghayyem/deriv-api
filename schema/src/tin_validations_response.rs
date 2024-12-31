
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/tin_validations/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// A message with validations for Tax Identification Numbers (TIN)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct TinValidationsResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// Validations for Tax Identification Numbers (TIN)
    #[serde(rename = "tin_validations", skip_serializing_if = "Option::is_none")]
    pub tin_validations: TinValidations,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Validations for Tax Identification Numbers (TIN)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct TinValidations {
    /// Invalid regex patterns for tin validation
    #[serde(rename = "invalid_patterns", skip_serializing_if = "Option::is_none")]
    pub invalid_patterns: Vec<String>,
    /// Whether the TIN is mandatory for the selected country
    #[serde(rename = "is_tin_mandatory", skip_serializing_if = "Option::is_none")]
    pub is_tin_mandatory: IsTinMandatoryEnum,
    /// List of employment statuses that bypass TIN requirements for the selected country
    #[serde(rename = "tin_employment_status_bypass", skip_serializing_if = "Option::is_none")]
    pub tin_employment_status_bypass: Vec<String>,
    /// Country tax identifier formats.
    #[serde(rename = "tin_format", skip_serializing_if = "Option::is_none")]
    pub tin_format: Vec<String>,
}




/// Whether the TIN is mandatory for the selected country
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IsTinMandatoryEnum {
    Value0,
    Value1 = 1,
}

impl IsTinMandatoryEnum {
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


