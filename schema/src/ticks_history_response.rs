
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/ticks_history/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Historic tick data for a single symbol
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct TicksHistoryResponse {
    /// Array of OHLC (open/high/low/close) price values for the given time (only for style=`candles`)
    #[serde(rename = "candles", skip_serializing_if = "Option::is_none")]
    pub candles: Vec<Candlesitem>,
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Historic tick data for a given symbol. Note: this will always return the latest possible set of ticks with accordance to the parameters specified.
    #[serde(rename = "history", skip_serializing_if = "Option::is_none")]
    pub history: History,
    /// Type of the response according to the `style` sent in request. Would be `history` or `candles` for the first response, and `tick` or `ohlc` for the rest when subscribed.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Historic tick data for a given symbol. Note: this will always return the latest possible set of ticks with accordance to the parameters specified.
    #[serde(rename = "ohlc", skip_serializing_if = "Option::is_none")]
    pub ohlc: Ohlc,
    /// Indicates the number of decimal points that the returned amounts must be displayed with
    #[serde(rename = "pip_size", skip_serializing_if = "Option::is_none")]
    pub pip_size: f64,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// For subscription requests only.
    #[serde(rename = "subscription", skip_serializing_if = "Option::is_none")]
    pub subscription: Subscription,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;

use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Candlesitem {
    /// It is the close price value for the given time
    #[serde(rename = "close", skip_serializing_if = "Option::is_none")]
    pub close: f64,
    /// It is an epoch value
    #[serde(rename = "epoch", skip_serializing_if = "Option::is_none")]
    pub epoch: i64,
    /// It is the high price value for the given time
    #[serde(rename = "high", skip_serializing_if = "Option::is_none")]
    pub high: f64,
    /// It is the low price value for the given time
    #[serde(rename = "low", skip_serializing_if = "Option::is_none")]
    pub low: f64,
    /// It is the open price value for the given time
    #[serde(rename = "open", skip_serializing_if = "Option::is_none")]
    pub open: f64,
}






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Historic tick data for a given symbol. Note: this will always return the latest possible set of ticks with accordance to the parameters specified.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct History {
    /// An array containing list of tick values for the corresponding epoch values in `times` array.
    #[serde(rename = "prices", skip_serializing_if = "Option::is_none")]
    pub prices: Vec<f64>,
    /// An array containing list of epoch values for the corresponding tick values in `prices` array.
    #[serde(rename = "times", skip_serializing_if = "Option::is_none")]
    pub times: Vec<i64>,
}






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;

use chrono::{DateTime, Utc};

/// Historic tick data for a given symbol. Note: this will always return the latest possible set of ticks with accordance to the parameters specified.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Ohlc {
    /// It is the close price value for the given time
    #[serde(rename = "close", skip_serializing_if = "Option::is_none")]
    pub close: String,
    /// It is an epoch value
    #[serde(rename = "epoch", skip_serializing_if = "Option::is_none")]
    pub epoch: i64,
    /// Granularity
    #[serde(rename = "granularity", skip_serializing_if = "Option::is_none")]
    pub granularity: i64,
    /// It is the high price value for the given time
    #[serde(rename = "high", skip_serializing_if = "Option::is_none")]
    pub high: String,
    /// Subscription unique identifier. Can be passed to the `forget` API call to unsubscribe.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    /// It is the low price value for the given time
    #[serde(rename = "low", skip_serializing_if = "Option::is_none")]
    pub low: String,
    /// It is the open price value for the given time
    #[serde(rename = "open", skip_serializing_if = "Option::is_none")]
    pub open: String,
    /// It is an epoch of open time
    #[serde(rename = "open_time", skip_serializing_if = "Option::is_none")]
    pub open_time: i64,
    /// PIP size
    #[serde(rename = "pip_size", skip_serializing_if = "Option::is_none")]
    pub pip_size: i64,
    /// Symbol name
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: String,
}






