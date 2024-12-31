
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/landing_company_details/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// The company has a number of licensed subsidiaries in various jurisdictions, which are called Landing Companies (and which are wholly owned subsidiaries of the Deriv Group). This call provides information about each Landing Company.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct LandingCompanyDetailsRequest {
    /// [Optional] Will return an extra field `tin_not_mandatory` indicating if the landing company does not require tax identification number for the provided country.
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: String,
    /// Landing company shortcode.
    #[serde(rename = "landing_company_details")]
    pub landing_company_details: LandingCompanyDetailsEnum,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
}




/// Landing company shortcode.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LandingCompanyDetailsEnum {
    Iom,
    Malta,
    Maltainvest,
    Svg,
    Virtual,
    Vanuatu,
    Samoa,
    Samoavirtual,
    Dsl,
    Bvi,
    Labuan,
    Dml,
}

impl LandingCompanyDetailsEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Iom => "iom",
            Self::Malta => "malta",
            Self::Maltainvest => "maltainvest",
            Self::Svg => "svg",
            Self::Virtual => "virtual",
            Self::Vanuatu => "vanuatu",
            Self::Samoa => "samoa",
            Self::Samoavirtual => "samoa-virtual",
            Self::Dsl => "dsl",
            Self::Bvi => "bvi",
            Self::Labuan => "labuan",
            Self::Dml => "dml",
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
            "samoa" => Some(Self::Samoa),
            "samoa-virtual" => Some(Self::Samoavirtual),
            "dsl" => Some(Self::Dsl),
            "bvi" => Some(Self::Bvi),
            "labuan" => Some(Self::Labuan),
            "dml" => Some(Self::Dml),
            _ => None,
        }
    }
}
