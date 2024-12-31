
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/authorize/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Authorize current WebSocket session to act on behalf of the owner of a given token. Must precede requests that need to access client account, for example purchasing and selling contracts or viewing portfolio.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct AuthorizeRequest {
    /// [Optional] Send this when you use api tokens for authorization and want to track activity using `login_history` call.
    #[serde(rename = "add_to_login_history", skip_serializing_if = "Option::is_none")]
    pub add_to_login_history: AddToLoginHistoryEnum,
    /// Authentication token. May be retrieved from https://www.binary.com/en/user/security/api_tokenws.html. Set to MULTI when using multiple tokens.
    #[serde(rename = "authorize")]
    pub authorize: String,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// Additional Authentication tokens of authorized user that may be used in this session. Upto 25 tokens.
    #[serde(rename = "tokens", skip_serializing_if = "Option::is_none")]
    pub tokens: Vec<String>,
}




/// [Optional] Send this when you use api tokens for authorization and want to track activity using `login_history` call.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AddToLoginHistoryEnum {
    Value1 = 1,
    Value0,
}

impl AddToLoginHistoryEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Value1 => "1",
            Self::Value0 => "0",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "1" => Some(Self::Value1),
            "0" => Some(Self::Value0),
            _ => None,
        }
    }
}
