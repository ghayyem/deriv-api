
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/trading_durations/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// A message with trading duration information for symbol and contract combinations.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct TradingDurationsResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// List of underlyings by their display name and symbol followed by their available contract types and trading duration boundaries.
    #[serde(rename = "trading_durations", skip_serializing_if = "Option::is_none")]
    pub trading_durations: Vec<TradingDurationsitem>,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct TradingDurationsitem {
    /// Available contract types and trading duration boundaries
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Vec<Dataitem>,
    /// The market in which the underlyings listed in `symbol` located.
    #[serde(rename = "market", skip_serializing_if = "Option::is_none")]
    pub market: Market,
    /// The submarket in which the underlyings listed in `symbol` located.
    #[serde(rename = "submarket", skip_serializing_if = "Option::is_none")]
    pub submarket: Submarket,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Dataitem {
    /// The market in which the underlyings listed in `symbol` located.
    #[serde(rename = "market", skip_serializing_if = "Option::is_none")]
    pub market: Market,
    /// The submarket in which the underlyings listed in `symbol` located.
    #[serde(rename = "submarket", skip_serializing_if = "Option::is_none")]
    pub submarket: Submarket,
    /// List of underlying symbols.
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Vec<Symbolitem>,
    /// List of trade durations available for symbols and contract combinations.
    #[serde(rename = "trade_durations", skip_serializing_if = "Option::is_none")]
    pub trade_durations: Vec<TradeDurationsitem>,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// The market in which the underlyings listed in `symbol` located.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Market {
    /// Translated market name.
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: String,
    /// Market name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
}






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// The submarket in which the underlyings listed in `symbol` located.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Submarket {
    /// Translated submarket name.
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: String,
    /// Submarket name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
}






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Symbolitem {
    /// Translated symbol name.
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: String,
    /// Symbol name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
}






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct TradeDurationsitem {
    /// List of trade durations available for the symbols.
    #[serde(rename = "durations", skip_serializing_if = "Option::is_none")]
    pub durations: Vec<Durationsitem>,
    /// List of trade types available for the symbols.
    #[serde(rename = "trade_type", skip_serializing_if = "Option::is_none")]
    pub trade_type: TradeType,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Durationsitem {
    /// Translated duration type name.
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: String,
    /// Maximum allowed duration for this type.
    #[serde(rename = "max", skip_serializing_if = "Option::is_none")]
    pub max: i64,
    /// Minimum allowed duration for this type.
    #[serde(rename = "min", skip_serializing_if = "Option::is_none")]
    pub min: i64,
    /// Duration type name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
}






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// List of trade types available for the symbols.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct TradeType {
    /// Translated trade type name.
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: String,
    /// Trade type name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
}












