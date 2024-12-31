
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/api_token/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// The result of the API token request made.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct ApiTokenResponse {
    /// Contains the result of API token according to the type of request.
    #[serde(rename = "api_token", skip_serializing_if = "Option::is_none")]
    pub api_token: ApiToken,
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



/// Contains the result of API token according to the type of request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct ApiToken {
    /// Token deleted.
    #[serde(rename = "delete_token", skip_serializing_if = "Option::is_none")]
    pub delete_token: DeleteTokenEnum,
    /// Token created.
    #[serde(rename = "new_token", skip_serializing_if = "Option::is_none")]
    pub new_token: NewTokenEnum,
    /// API tokens
    #[serde(rename = "tokens", skip_serializing_if = "Option::is_none")]
    pub tokens: Vec<Tokensitem>,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// The information for each token.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Tokensitem {
    /// The token name specified when creating.
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: String,
    /// The last date which the token has been used.
    #[serde(rename = "last_used", skip_serializing_if = "Option::is_none")]
    pub last_used: String,
    /// List of permission scopes of the token.
    #[serde(rename = "scopes", skip_serializing_if = "Option::is_none")]
    pub scopes: Vec<ScopesitemEnum>,
    /// The token that can be used to `authorize` with.
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: String,
    /// The IP restriction for the token. No restriction if empty.
    #[serde(rename = "valid_for_ip", skip_serializing_if = "Option::is_none")]
    pub valid_for_ip: String,
}






/// Token deleted.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeleteTokenEnum {
    Value1 = 1,
}

impl DeleteTokenEnum {
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
/// Token created.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NewTokenEnum {
    Value1 = 1,
}

impl NewTokenEnum {
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


