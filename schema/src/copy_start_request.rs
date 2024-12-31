
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/copy_start/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Start copy trader bets
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct CopyStartRequest {
    /// [Optional] Used to set assets to be copied. E.x ["frxUSDJPY", "R_50"]
    #[serde(rename = "assets", skip_serializing_if = "Option::is_none")]
    pub assets: Value,
    /// API tokens identifying the accounts of trader which will be used to copy trades
    #[serde(rename = "copy_start")]
    pub copy_start: String,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// [Optional] Used to set maximum trade stake to be copied.
    #[serde(rename = "max_trade_stake", skip_serializing_if = "Option::is_none")]
    pub max_trade_stake: f64,
    /// [Optional] Used to set minimal trade stake to be copied.
    #[serde(rename = "min_trade_stake", skip_serializing_if = "Option::is_none")]
    pub min_trade_stake: f64,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// [Optional] Used to set trade types to be copied. E.x ["CALL", "PUT"]
    #[serde(rename = "trade_types", skip_serializing_if = "Option::is_none")]
    pub trade_types: Value,
}




