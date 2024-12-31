
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/app_list/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// A message with created applications
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct AppListResponse {
    /// List of created applications for the authorized account.
    #[serde(rename = "app_list", skip_serializing_if = "Option::is_none")]
    pub app_list: Vec<AppListitem>,
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



#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct AppListitem {
    /// Active.
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: i64,
    /// Application ID.
    #[serde(rename = "app_id")]
    pub app_id: i64,
    /// Markup added to contract prices (as a percentage of contract payout).
    #[serde(rename = "app_markup_percentage")]
    pub app_markup_percentage: f64,
    /// Application's App Store URL.
    #[serde(rename = "appstore", skip_serializing_if = "Option::is_none")]
    pub appstore: Option<Value>,
    /// Application's GitHub page. (for open-source projects)
    #[serde(rename = "github", skip_serializing_if = "Option::is_none")]
    pub github: Option<Value>,
    /// Application's Google Play URL.
    #[serde(rename = "googleplay", skip_serializing_if = "Option::is_none")]
    pub googleplay: Option<Value>,
    /// Application's homepage URL.
    #[serde(rename = "homepage", skip_serializing_if = "Option::is_none")]
    pub homepage: Option<Value>,
    /// Application name.
    #[serde(rename = "name")]
    pub name: String,
    /// The URL to redirect to after a successful login.
    #[serde(rename = "redirect_uri")]
    pub redirect_uri: String,
    /// Scope Details.
    #[serde(rename = "scopes", skip_serializing_if = "Option::is_none")]
    pub scopes: Vec<String>,
    /// Used when `verify_email` called. If available, a URL containing the verification token will send to the client's email, otherwise only the token will be sent.
    #[serde(rename = "verification_uri", skip_serializing_if = "Option::is_none")]
    pub verification_uri: Option<Value>,
}






