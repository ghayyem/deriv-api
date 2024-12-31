
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/ticks_history/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Get historic tick data for a given symbol.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct TicksHistoryRequest {
    /// [Optional] 1 - if the market is closed at the end time, or license limit is before end time, adjust interval backwards to compensate.
    #[serde(rename = "adjust_start_time", skip_serializing_if = "Option::is_none")]
    pub adjust_start_time: AdjustStartTimeEnum,
    /// [Optional] An upper limit on ticks to receive.
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: i64,
    /// Epoch value representing the latest boundary of the returned ticks. If `latest` is specified, this will be the latest available timestamp.
    #[serde(rename = "end")]
    pub end: String,
    /// [Optional] Only applicable for style: `candles`. Candle time-dimension width setting. (default: `60`).
    #[serde(rename = "granularity", skip_serializing_if = "Option::is_none")]
    pub granularity: GranularityEnum,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// [Optional] Epoch value representing the earliest boundary of the returned ticks. 
/// - For `"style": "ticks"`: this will default to 1 day ago.
/// - For `"style": "candles"`: it will default to 1 day ago if count or granularity is undefined.
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: i64,
    /// [Optional] The tick-output style.
    #[serde(rename = "style", skip_serializing_if = "Option::is_none")]
    pub style: StyleEnum,
    /// [Optional] 1 - to send updates whenever a new tick is received.
    #[serde(rename = "subscribe", skip_serializing_if = "Option::is_none")]
    pub subscribe: SubscribeEnum,
    /// Short symbol name (obtained from the `active_symbols` call).
    #[serde(rename = "ticks_history")]
    pub ticks_history: String,
}




/// [Optional] 1 - if the market is closed at the end time, or license limit is before end time, adjust interval backwards to compensate.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdjustStartTimeEnum {
    Value1 = 1,
}

impl AdjustStartTimeEnum {
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
/// [Optional] Only applicable for style: `candles`. Candle time-dimension width setting. (default: `60`).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GranularityEnum {
    Value60 = 60,
    Value120 = 120,
    Value180 = 180,
    Value300 = 300,
    Value600 = 600,
    Value900 = 900,
    Value1800 = 1800,
    Value3600 = 3600,
    Value7200 = 7200,
    Value14400 = 14400,
    Value28800 = 28800,
    Value86400 = 86400,
}

impl GranularityEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Value60 => "60",
            Self::Value120 => "120",
            Self::Value180 => "180",
            Self::Value300 => "300",
            Self::Value600 => "600",
            Self::Value900 => "900",
            Self::Value1800 => "1800",
            Self::Value3600 => "3600",
            Self::Value7200 => "7200",
            Self::Value14400 => "14400",
            Self::Value28800 => "28800",
            Self::Value86400 => "86400",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "60" => Some(Self::Value60),
            "120" => Some(Self::Value120),
            "180" => Some(Self::Value180),
            "300" => Some(Self::Value300),
            "600" => Some(Self::Value600),
            "900" => Some(Self::Value900),
            "1800" => Some(Self::Value1800),
            "3600" => Some(Self::Value3600),
            "7200" => Some(Self::Value7200),
            "14400" => Some(Self::Value14400),
            "28800" => Some(Self::Value28800),
            "86400" => Some(Self::Value86400),
            _ => None,
        }
    }
}
/// [Optional] The tick-output style.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StyleEnum {
    Candles,
    Ticks,
}

impl StyleEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Candles => "candles",
            Self::Ticks => "ticks",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "candles" => Some(Self::Candles),
            "ticks" => Some(Self::Ticks),
            _ => None,
        }
    }
}
