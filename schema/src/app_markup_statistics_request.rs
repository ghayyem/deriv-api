
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/app_markup_statistics/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Retrieve statistics of `app_markup`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct AppMarkupStatisticsRequest {
    /// Must be `1`
    #[serde(rename = "app_markup_statistics")]
    pub app_markup_statistics: AppMarkupStatisticsEnum,
    /// Start date (epoch or YYYY-MM-DD HH:MM:SS). Results are inclusive of this time.
    #[serde(rename = "date_from")]
    pub date_from: String,
    /// End date (epoch or YYYY-MM-DD HH::MM::SS). Results are inclusive of this time.
    #[serde(rename = "date_to")]
    pub date_to: String,
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
pub enum AppMarkupStatisticsEnum {
    Value1 = 1,
}

impl AppMarkupStatisticsEnum {
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
