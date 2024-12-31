
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/partner_settings_update/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// A message with Partner Settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct PartnerSettingsUpdateRequest {
    /// [Optional] Company name. Only applicable for partners of type company.
    #[serde(rename = "company_name", skip_serializing_if = "Option::is_none")]
    pub company_name: String,
    /// [Optional] Company registration number. Only applicable for partners of type company.
    #[serde(rename = "company_registration_no", skip_serializing_if = "Option::is_none")]
    pub company_registration_no: String,
    /// [Optional] The login id of the partner account. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// Must be `1`
    #[serde(rename = "partner_settings_update")]
    pub partner_settings_update: PartnerSettingsUpdateEnum,
    /// Defines whether this partner is an individual or a company.
    #[serde(rename = "partner_type", skip_serializing_if = "Option::is_none")]
    pub partner_type: PartnerTypeEnum,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// Partner's Website URI/Promotional Platform
    #[serde(rename = "website", skip_serializing_if = "Option::is_none")]
    pub website: String,
}




/// Must be `1`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PartnerSettingsUpdateEnum {
    Value1 = 1,
}

impl PartnerSettingsUpdateEnum {
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
/// Defines whether this partner is an individual or a company.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PartnerTypeEnum {
    Individual,
    Company,
}

impl PartnerTypeEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Individual => "individual",
            Self::Company => "company",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "individual" => Some(Self::Individual),
            "company" => Some(Self::Company),
            _ => None,
        }
    }
}
