
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/api_token/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// This call manages API tokens
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct ApiTokenRequest {
    /// Must be `1`
    #[serde(rename = "api_token")]
    pub api_token: ApiTokenEnum,
    /// [Optional] The token to remove.
    #[serde(rename = "delete_token", skip_serializing_if = "Option::is_none")]
    pub delete_token: String,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// [Optional] The name of the created token.
    #[serde(rename = "new_token", skip_serializing_if = "Option::is_none")]
    pub new_token: String,
    /// [Optional] List of permission scopes to provide with the token.
    #[serde(rename = "new_token_scopes", skip_serializing_if = "Option::is_none")]
    pub new_token_scopes: Vec<NewTokenScopesitemEnum>,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// [Optional] If you set this parameter during token creation, then the token created will only work for the IP address that was used to create the token
    #[serde(rename = "valid_for_current_ip_only", skip_serializing_if = "Option::is_none")]
    pub valid_for_current_ip_only: ValidForCurrentIpOnlyEnum,
}




/// Must be `1`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApiTokenEnum {
    Value1 = 1,
}

impl ApiTokenEnum {
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
/// Required when create new token
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NewTokenScopesitemEnum {
    Read,
    Trade,
    Trading_Information,
    Payments,
    Admin,
}

impl NewTokenScopesitemEnum {
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
/// [Optional] If you set this parameter during token creation, then the token created will only work for the IP address that was used to create the token
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidForCurrentIpOnlyEnum {
    Value0,
    Value1 = 1,
}

impl ValidForCurrentIpOnlyEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Value0 => "0",
            Self::Value1 => "1",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "0" => Some(Self::Value0),
            "1" => Some(Self::Value1),
            _ => None,
        }
    }
}
