
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/revoke_oauth_app/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Used for revoking access of particular app.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct RevokeOauthAppRequest {
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// The application ID to revoke.
    #[serde(rename = "revoke_oauth_app")]
    pub revoke_oauth_app: i64,
}




