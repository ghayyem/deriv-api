
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/ticks_batch/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Initiate a continuous stream of spot price updates for a group symbols.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct TicksBatchRequest {
    /// The short market name.
    #[serde(rename = "market")]
    pub market: MarketEnum,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// [Optional] If set to 1, will send updates in batches by market.
    #[serde(rename = "subscribe", skip_serializing_if = "Option::is_none")]
    pub subscribe: SubscribeEnum,
    /// Must be `1`
    #[serde(rename = "ticks_batch")]
    pub ticks_batch: TicksBatchEnum,
}




/// The short market name.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarketEnum {
    Forex,
    Indices,
    Commodities,
    Stocks,
    Cryptocurrency,
    Synthetic_Index,
}

impl MarketEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Forex => "forex",
            Self::Indices => "indices",
            Self::Commodities => "commodities",
            Self::Stocks => "stocks",
            Self::Cryptocurrency => "cryptocurrency",
            Self::Synthetic_Index => "synthetic_index",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "forex" => Some(Self::Forex),
            "indices" => Some(Self::Indices),
            "commodities" => Some(Self::Commodities),
            "stocks" => Some(Self::Stocks),
            "cryptocurrency" => Some(Self::Cryptocurrency),
            "synthetic_index" => Some(Self::Synthetic_Index),
            _ => None,
        }
    }
}
/// Must be `1`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TicksBatchEnum {
    Value1 = 1,
}

impl TicksBatchEnum {
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
