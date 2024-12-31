
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/mt5_password_reset/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// To reset the password of MT5 account.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Mt5PasswordResetRequest {
    /// MT5 user login
    #[serde(rename = "login")]
    pub login: String,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// Must be `1`
    #[serde(rename = "mt5_password_reset")]
    pub mt_5_password_reset: Mt5PasswordResetEnum,
    /// New password of the account. For validation (Accepts any printable ASCII character. Must be within 8-25 characters, and include numbers, lowercase and uppercase letters. Must not be the same as the user's email address).
    #[serde(rename = "new_password")]
    pub new_password: String,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] Type of the password to reset.
    #[serde(rename = "password_type", skip_serializing_if = "Option::is_none")]
    pub password_type: PasswordTypeEnum,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// Email verification code (received from a `verify_email` call, which must be done first)
    #[serde(rename = "verification_code")]
    pub verification_code: String,
}




/// Must be `1`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mt5PasswordResetEnum {
    Value1 = 1,
}

impl Mt5PasswordResetEnum {
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
/// [Optional] Type of the password to reset.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PasswordTypeEnum {
    Main,
    Investor,
}

impl PasswordTypeEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Main => "main",
            Self::Investor => "investor",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "main" => Some(Self::Main),
            "investor" => Some(Self::Investor),
            _ => None,
        }
    }
}
