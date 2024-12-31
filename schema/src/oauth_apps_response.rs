
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/oauth_apps/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// A message with used applications
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct OauthAppsResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// List of 3rd party OAuth applications that used for the authorized account.
    #[serde(rename = "oauth_apps", skip_serializing_if = "Option::is_none")]
    pub oauth_apps: Vec<OauthAppsitem>,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct OauthAppsitem {
    /// Application ID.
    #[serde(rename = "app_id")]
    pub app_id: i64,
    /// Markup added to contract prices (as a percentage of contract payout)
    #[serde(rename = "app_markup_percentage")]
    pub app_markup_percentage: f64,
    /// The last date which the application has been used.
    #[serde(rename = "last_used", skip_serializing_if = "Option::is_none")]
    pub last_used: Option<Value>,
    /// Application name
    #[serde(rename = "name")]
    pub name: String,
    /// Boolean value: 1 or 0, indicating 1 if app is an official app and 0 incase of unofficial app
    #[serde(rename = "official")]
    pub official: OfficialEnum,
    /// The list of permission scopes grant for each app.
    #[serde(rename = "scopes")]
    pub scopes: Vec<String>,
}




/// Boolean value: 1 or 0, indicating 1 if app is an official app and 0 incase of unofficial app
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OfficialEnum {
    Value1 = 1,
    Value0,
}

impl OfficialEnum {
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


