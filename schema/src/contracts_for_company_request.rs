
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/contracts_for_company/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Get the list of currently available contracts for a given landing company.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct ContractsForCompanyRequest {
    /// Must be `1`
    #[serde(rename = "contracts_for_company")]
    pub contracts_for_company: ContractsForCompanyEnum,
    /// [Optional] Indicates which landing company to get a list of contracts for. If you are logged in, your account's landing company will override this field.
    #[serde(rename = "landing_company", skip_serializing_if = "Option::is_none")]
    pub landing_company: LandingCompanyEnum,
    /// [Optional] The login id of the user. If left unspecified, it defaults to the initial authorized token's login id.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
}




/// Must be `1`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContractsForCompanyEnum {
    Value1 = 1,
}

impl ContractsForCompanyEnum {
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
/// [Optional] Indicates which landing company to get a list of contracts for. If you are logged in, your account's landing company will override this field.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LandingCompanyEnum {
    Iom,
    Malta,
    Maltainvest,
    Svg,
    Virtual,
    Vanuatu,
}

impl LandingCompanyEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Iom => "iom",
            Self::Malta => "malta",
            Self::Maltainvest => "maltainvest",
            Self::Svg => "svg",
            Self::Virtual => "virtual",
            Self::Vanuatu => "vanuatu",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "iom" => Some(Self::Iom),
            "malta" => Some(Self::Malta),
            "maltainvest" => Some(Self::Maltainvest),
            "svg" => Some(Self::Svg),
            "virtual" => Some(Self::Virtual),
            "vanuatu" => Some(Self::Vanuatu),
            _ => None,
        }
    }
}
