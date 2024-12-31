
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/app_update/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// A message with created application
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct AppUpdateResponse {
    /// Information of the updated application.
    #[serde(rename = "app_update", skip_serializing_if = "Option::is_none")]
    pub app_update: AppUpdate,
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



/// Information of the updated application.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct AppUpdate {
    /// Active.
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: i64,
    /// Application ID.
    #[serde(rename = "app_id", skip_serializing_if = "Option::is_none")]
    pub app_id: i64,
    /// Markup added to contract prices (as a percentage of contract payout).
    #[serde(rename = "app_markup_percentage", skip_serializing_if = "Option::is_none")]
    pub app_markup_percentage: f64,
    /// Application's App Store URL.
    #[serde(rename = "appstore", skip_serializing_if = "Option::is_none")]
    pub appstore: String,
    /// Application's GitHub page (for open-source projects).
    #[serde(rename = "github", skip_serializing_if = "Option::is_none")]
    pub github: String,
    /// Application's Google Play URL.
    #[serde(rename = "googleplay", skip_serializing_if = "Option::is_none")]
    pub googleplay: String,
    /// Application's homepage URL.
    #[serde(rename = "homepage", skip_serializing_if = "Option::is_none")]
    pub homepage: String,
    /// Application name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// The URL to redirect to after a successful login.
    #[serde(rename = "redirect_uri", skip_serializing_if = "Option::is_none")]
    pub redirect_uri: String,
    /// Scope Details.
    #[serde(rename = "scopes", skip_serializing_if = "Option::is_none")]
    pub scopes: Vec<String>,
    /// Used when `verify_email` called. If available, a URL containing the verification token will be sent to the client's email, otherwise only the token will be sent.
    #[serde(rename = "verification_uri", skip_serializing_if = "Option::is_none")]
    pub verification_uri: String,
}






