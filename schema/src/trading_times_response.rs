
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/trading_times/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// A message with Trading Times
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct TradingTimesResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// The trading times structure is a hierarchy as follows: Market -> SubMarket -> Underlyings
    #[serde(rename = "trading_times", skip_serializing_if = "Option::is_none")]
    pub trading_times: TradingTimes,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// The trading times structure is a hierarchy as follows: Market -> SubMarket -> Underlyings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct TradingTimes {
    /// An array of markets
    #[serde(rename = "markets")]
    pub markets: Vec<Marketsitem>,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Marketsitem {
    /// Market name
    #[serde(rename = "name")]
    pub name: String,
    /// An array of submarkets
    #[serde(rename = "submarkets", skip_serializing_if = "Option::is_none")]
    pub submarkets: Vec<Submarketsitem>,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Submarketsitem {
    /// Submarket name
    #[serde(rename = "name")]
    pub name: String,
    /// Symbols array
    #[serde(rename = "symbols")]
    pub symbols: Vec<Symbolsitem>,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Symbolsitem {
    /// Events
    #[serde(rename = "events", skip_serializing_if = "Option::is_none")]
    pub events: Vec<Value>,
    /// Symbol name
    #[serde(rename = "name")]
    pub name: String,
    /// Symbol shortcode
    #[serde(rename = "symbol")]
    pub symbol: String,
    /// Open, close and settlement times
    #[serde(rename = "times", skip_serializing_if = "Option::is_none")]
    pub times: Times,
    /// Trading days
    #[serde(rename = "trading_days", skip_serializing_if = "Option::is_none")]
    pub trading_days: Vec<TradingDaysitemEnum>,
}




/// 
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TradingDaysitemEnum {
    Sun,
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
}

impl TradingDaysitemEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Sun => "Sun",
            Self::Mon => "Mon",
            Self::Tue => "Tue",
            Self::Wed => "Wed",
            Self::Thu => "Thu",
            Self::Fri => "Fri",
            Self::Sat => "Sat",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Sun" => Some(Self::Sun),
            "Mon" => Some(Self::Mon),
            "Tue" => Some(Self::Tue),
            "Wed" => Some(Self::Wed),
            "Thu" => Some(Self::Thu),
            "Fri" => Some(Self::Fri),
            "Sat" => Some(Self::Sat),
            _ => None,
        }
    }
}








