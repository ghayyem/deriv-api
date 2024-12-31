
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/app_update/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Update a new OAuth application
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct AppUpdateRequest {
    /// [Optional] Markup to be added to contract prices (as a percentage of contract payout). Max markup: 3%.
    #[serde(rename = "app_markup_percentage", skip_serializing_if = "Option::is_none")]
    pub app_markup_percentage: f64,
    /// Application app_id.
    #[serde(rename = "app_update")]
    pub app_update: i64,
    /// [Optional] Application's App Store URL (if applicable).
    #[serde(rename = "appstore", skip_serializing_if = "Option::is_none")]
    pub appstore: String,
    /// [Optional] Application's GitHub page (for open-source projects).
    #[serde(rename = "github", skip_serializing_if = "Option::is_none")]
    pub github: String,
    /// [Optional] Application's Google Play URL (if applicable).
    #[serde(rename = "googleplay", skip_serializing_if = "Option::is_none")]
    pub googleplay: String,
    /// [Optional] Application's homepage URL.
    #[serde(rename = "homepage", skip_serializing_if = "Option::is_none")]
    pub homepage: String,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// Application name.
    #[serde(rename = "name")]
    pub name: String,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] The URL to redirect to after a successful login. Required if charging markup percentage.
    #[serde(rename = "redirect_uri", skip_serializing_if = "Option::is_none")]
    pub redirect_uri: String,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// Change scopes will revoke all user's grants and log them out.
    #[serde(rename = "scopes")]
    pub scopes: Vec<ScopesitemEnum>,
    /// [Optional] Used when `verify_email` called. If available, a URL containing the verification token will send to the client's email, otherwise only the token will be sent.
    #[serde(rename = "verification_uri", skip_serializing_if = "Option::is_none")]
    pub verification_uri: String,
}




/// 
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopesitemEnum {
    Read,
    Trade,
    Trading_Information,
    Payments,
    Admin,
}

impl ScopesitemEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Read => "read",
            Self::Trade => "trade",
            Self::Trading_Information => "trading_information",
            Self::Payments => "payments",
            Self::Admin => "admin",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "read" => Some(Self::Read),
            "trade" => Some(Self::Trade),
            "trading_information" => Some(Self::Trading_Information),
            "payments" => Some(Self::Payments),
            "admin" => Some(Self::Admin),
            _ => None,
        }
    }
}
