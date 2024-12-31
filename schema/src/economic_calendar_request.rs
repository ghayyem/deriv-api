
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/economic_calendar/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Specify a currency to receive a list of events related to that specific currency. For example, specifying USD will return a list of USD-related events. If the currency is omitted, you will receive a list for all currencies.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct EconomicCalendarRequest {
    /// [Optional] Currency symbol.
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: String,
    /// Must be `1`
    #[serde(rename = "economic_calendar")]
    pub economic_calendar: EconomicCalendarEnum,
    /// [Optional] End date.
    #[serde(rename = "end_date", skip_serializing_if = "Option::is_none")]
    pub end_date: i64,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// [Optional] Start date.
    #[serde(rename = "start_date", skip_serializing_if = "Option::is_none")]
    pub start_date: i64,
}




/// Must be `1`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EconomicCalendarEnum {
    Value1 = 1,
}

impl EconomicCalendarEnum {
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
