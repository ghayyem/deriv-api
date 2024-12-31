
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/copytrading_list/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Details of copiers and/or traders for Copy Trading
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct CopytradingListResponse {
    /// The trading information of copiers or traders.
    #[serde(rename = "copytrading_list", skip_serializing_if = "Option::is_none")]
    pub copytrading_list: CopytradingList,
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



/// The trading information of copiers or traders.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct CopytradingList {
    /// List of users who are currently copy trading the authenticated user
    #[serde(rename = "copiers")]
    pub copiers: Vec<Copiersitem>,
    /// List of traders being followed by the authenticated user
    #[serde(rename = "traders")]
    pub traders: Vec<Tradersitem>,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Copiersitem {
    /// The loginid of the copier's account.
    #[serde(rename = "loginid")]
    pub loginid: String,
}






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Tradersitem {
    /// The list of assets to copy the trades of.
    #[serde(rename = "assets", skip_serializing_if = "Option::is_none")]
    pub assets: Vec<String>,
    /// The loginid of the trader's account.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// Maximum trading stake set for the trader.
    #[serde(rename = "max_trade_stake", skip_serializing_if = "Option::is_none")]
    pub max_trade_stake: Option<Value>,
    /// Minimum trading stake set for the trader.
    #[serde(rename = "min_trade_stake", skip_serializing_if = "Option::is_none")]
    pub min_trade_stake: Option<Value>,
    /// The token provided for the trader.
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: String,
    /// The type of trades set.
    #[serde(rename = "trade_types", skip_serializing_if = "Option::is_none")]
    pub trade_types: Vec<String>,
}








