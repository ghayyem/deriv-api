
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/active_symbols/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// A message containing the list of active symbols.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct ActiveSymbolsResponse {
    /// List of active symbols.
    #[serde(rename = "active_symbols", skip_serializing_if = "Option::is_none")]
    pub active_symbols: Vec<ActiveSymbolsitem>,
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// The information about each symbol.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct ActiveSymbolsitem {
    /// `1` if the symbol is tradable in a forward starting contract, `0` if not.
    #[serde(rename = "allow_forward_starting", skip_serializing_if = "Option::is_none")]
    pub allow_forward_starting: AllowForwardStartingEnum,
    /// Amount the data feed is delayed (in minutes) due to Exchange licensing requirements. Only returned on `full` active symbols call.
    #[serde(rename = "delay_amount", skip_serializing_if = "Option::is_none")]
    pub delay_amount: i64,
    /// Display name.
    #[serde(rename = "display_name")]
    pub display_name: String,
    /// Display order.
    #[serde(rename = "display_order")]
    pub display_order: i64,
    /// `1` if market is currently open, `0` if closed.
    #[serde(rename = "exchange_is_open")]
    pub exchange_is_open: ExchangeIsOpenEnum,
    /// Exchange name (for underlyings listed on a Stock Exchange). Only returned on `full` active symbols call.
    #[serde(rename = "exchange_name", skip_serializing_if = "Option::is_none")]
    pub exchange_name: String,
    /// Intraday interval minutes. Only returned on `full` active symbols call.
    #[serde(rename = "intraday_interval_minutes", skip_serializing_if = "Option::is_none")]
    pub intraday_interval_minutes: i64,
    /// `1` indicates that trading is currently suspended, `0` if not.
    #[serde(rename = "is_trading_suspended")]
    pub is_trading_suspended: IsTradingSuspendedEnum,
    /// Market category (forex, indices, etc).
    #[serde(rename = "market")]
    pub market: String,
    /// Translated market name.
    #[serde(rename = "market_display_name")]
    pub market_display_name: String,
    /// Pip size (i.e. minimum fluctuation amount).
    #[serde(rename = "pip")]
    pub pip: f64,
    /// For stock indices, the underlying currency for that instrument. Only returned on `full` active symbols call.
    #[serde(rename = "quoted_currency_symbol", skip_serializing_if = "Option::is_none")]
    pub quoted_currency_symbol: String,
    /// Latest spot price of the underlying. Only returned on `full` active symbols call.
    #[serde(rename = "spot", skip_serializing_if = "Option::is_none")]
    pub spot: Option<Value>,
    /// Number of seconds elapsed since the last spot price. Only returned on `full` active symbols call.
    #[serde(rename = "spot_age", skip_serializing_if = "Option::is_none")]
    pub spot_age: String,
    /// Daily percentage for a symbol. Only returned on 'full' active symbols call.
    #[serde(rename = "spot_percentage_change", skip_serializing_if = "Option::is_none")]
    pub spot_percentage_change: String,
    /// Latest spot epoch time. Only returned on `full` active symbols call.
    #[serde(rename = "spot_time", skip_serializing_if = "Option::is_none")]
    pub spot_time: String,
    /// Subgroup name.
    #[serde(rename = "subgroup")]
    pub subgroup: String,
    /// Translated subgroup name.
    #[serde(rename = "subgroup_display_name")]
    pub subgroup_display_name: String,
    /// Submarket name.
    #[serde(rename = "submarket")]
    pub submarket: String,
    /// Translated submarket name.
    #[serde(rename = "submarket_display_name")]
    pub submarket_display_name: String,
    /// The symbol code for this underlying.
    #[serde(rename = "symbol")]
    pub symbol: String,
    /// Symbol type (forex, commodities, etc).
    #[serde(rename = "symbol_type")]
    pub symbol_type: String,
}




/// `1` if the symbol is tradable in a forward starting contract, `0` if not.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AllowForwardStartingEnum {
    Value0,
    Value1 = 1,
}

impl AllowForwardStartingEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Value0 => "0",
            Self::Value1 => "1",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "0" => Some(Self::Value0),
            "1" => Some(Self::Value1),
            _ => None,
        }
    }
}
/// `1` if market is currently open, `0` if closed.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExchangeIsOpenEnum {
    Value0,
    Value1 = 1,
}

impl ExchangeIsOpenEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Value0 => "0",
            Self::Value1 => "1",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "0" => Some(Self::Value0),
            "1" => Some(Self::Value1),
            _ => None,
        }
    }
}
/// `1` indicates that trading is currently suspended, `0` if not.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IsTradingSuspendedEnum {
    Value0,
    Value1 = 1,
}

impl IsTradingSuspendedEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Value0 => "0",
            Self::Value1 => "1",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "0" => Some(Self::Value0),
            "1" => Some(Self::Value1),
            _ => None,
        }
    }
}


