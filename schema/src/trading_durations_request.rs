
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/trading_durations/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Retrieve a list of all available underlyings and the corresponding contract types and trading duration boundaries. If the user is logged in, only the assets available for that user's landing company will be returned.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct TradingDurationsRequest {
    /// Deprecated - Replaced by landing_company_short.
    #[serde(rename = "landing_company", skip_serializing_if = "Option::is_none")]
    pub landing_company: LandingCompanyEnum,
    /// [Optional] If specified, will return only the underlyings for the specified landing company.
    #[serde(rename = "landing_company_short", skip_serializing_if = "Option::is_none")]
    pub landing_company_short: LandingCompanyShortEnum,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// Must be `1`
    #[serde(rename = "trading_durations")]
    pub trading_durations: TradingDurationsEnum,
}




/// Must be `1`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TradingDurationsEnum {
    Value1 = 1,
}

impl TradingDurationsEnum {
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
