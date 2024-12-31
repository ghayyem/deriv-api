
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/kyc_auth_status/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Get KYC Authentication Status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct KycAuthStatusRequest {
    /// The country for which service availability is being verified, 2-letter country code
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: String,
    /// Must be `1`
    #[serde(rename = "kyc_auth_status")]
    pub kyc_auth_status: KycAuthStatusEnum,
    /// Indicates which landing companies to get the KYC authentication status for.
    #[serde(rename = "landing_companies", skip_serializing_if = "Option::is_none")]
    pub landing_companies: Vec<LandingCompaniesitemEnum>,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
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
pub enum KycAuthStatusEnum {
    Value1 = 1,
}

impl KycAuthStatusEnum {
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
/// 
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LandingCompaniesitemEnum {
    Iom,
    Malta,
    Maltainvest,
    Svg,
    Virtual,
    Vanuatu,
    Labuan,
    Samoa,
    Samoavirtual,
    Bvi,
    Dsl,
}

impl LandingCompaniesitemEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Iom => "iom",
            Self::Malta => "malta",
            Self::Maltainvest => "maltainvest",
            Self::Svg => "svg",
            Self::Virtual => "virtual",
            Self::Vanuatu => "vanuatu",
            Self::Labuan => "labuan",
            Self::Samoa => "samoa",
            Self::Samoavirtual => "samoa-virtual",
            Self::Bvi => "bvi",
            Self::Dsl => "dsl",
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
            "labuan" => Some(Self::Labuan),
            "samoa" => Some(Self::Samoa),
            "samoa-virtual" => Some(Self::Samoavirtual),
            "bvi" => Some(Self::Bvi),
            "dsl" => Some(Self::Dsl),
            _ => None,
        }
    }
}
