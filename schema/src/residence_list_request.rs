
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/residence_list/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// This call returns a list of countries and 2-letter country codes, suitable for populating the account opening form.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct ResidenceListRequest {
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] Specific keys from the response that you want. If not passed, it will return all the keys.
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Vec<QueryitemEnum>,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// Must be `1`
    #[serde(rename = "residence_list")]
    pub residence_list: ResidenceListEnum,
}




/// Keys that you need back in response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryitemEnum {
    Account_Opening_Self_Declaration_Required,
    Common_Reporting_Standard,
    Disabled,
    Identity,
    Jurisdiction_Risk_Assessment,
    Phone_Idd,
    Selected,
    Text,
    Tin_Format,
    Value,
    Wallet_Signup,
}

impl QueryitemEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Account_Opening_Self_Declaration_Required => "account_opening_self_declaration_required",
            Self::Common_Reporting_Standard => "common_reporting_standard",
            Self::Disabled => "disabled",
            Self::Identity => "identity",
            Self::Jurisdiction_Risk_Assessment => "jurisdiction_risk_assessment",
            Self::Phone_Idd => "phone_idd",
            Self::Selected => "selected",
            Self::Text => "text",
            Self::Tin_Format => "tin_format",
            Self::Value => "value",
            Self::Wallet_Signup => "wallet_signup",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "account_opening_self_declaration_required" => Some(Self::Account_Opening_Self_Declaration_Required),
            "common_reporting_standard" => Some(Self::Common_Reporting_Standard),
            "disabled" => Some(Self::Disabled),
            "identity" => Some(Self::Identity),
            "jurisdiction_risk_assessment" => Some(Self::Jurisdiction_Risk_Assessment),
            "phone_idd" => Some(Self::Phone_Idd),
            "selected" => Some(Self::Selected),
            "text" => Some(Self::Text),
            "tin_format" => Some(Self::Tin_Format),
            "value" => Some(Self::Value),
            "wallet_signup" => Some(Self::Wallet_Signup),
            _ => None,
        }
    }
}
/// Must be `1`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResidenceListEnum {
    Value1 = 1,
}

impl ResidenceListEnum {
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
