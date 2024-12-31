
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/trading_servers/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Get the list of servers for a trading platform.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct TradingServersRequest {
    /// [Optional] Trading account type.
    #[serde(rename = "account_type", skip_serializing_if = "Option::is_none")]
    pub account_type: AccountTypeEnum,
    /// [Optional] Pass the environment (installation) instance. Currently, there are one demo and two real environments. Defaults to 'all'.
    #[serde(rename = "environment", skip_serializing_if = "Option::is_none")]
    pub environment: EnvironmentEnum,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// [Optional] Market type.
    #[serde(rename = "market_type", skip_serializing_if = "Option::is_none")]
    pub market_type: MarketTypeEnum,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] Pass the trading platform name, default to mt5
    #[serde(rename = "platform", skip_serializing_if = "Option::is_none")]
    pub platform: PlatformEnum,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// Must be `1`
    #[serde(rename = "trading_servers")]
    pub trading_servers: TradingServersEnum,
}




/// Must be `1`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TradingServersEnum {
    Value1 = 1,
}

impl TradingServersEnum {
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
