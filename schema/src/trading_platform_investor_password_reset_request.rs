
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/trading_platform_investor_password_reset/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Reset the investor password of a Trading Platform Account
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct TradingPlatformInvestorPasswordResetRequest {
    /// Trading account ID.
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// New password of the account. For validation (Accepts any printable ASCII character. Must be within 8-16 characters, include numbers, lowercase, uppercase letters and special characters. Must not be the same as the user's email address).
    #[serde(rename = "new_password")]
    pub new_password: String,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// Name of trading platform.
    #[serde(rename = "platform")]
    pub platform: PlatformEnum,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// Must be `1`
    #[serde(rename = "trading_platform_investor_password_reset")]
    pub trading_platform_investor_password_reset: TradingPlatformInvestorPasswordResetEnum,
    /// Email verification code (received from a `verify_email` call, which must be done first)
    #[serde(rename = "verification_code")]
    pub verification_code: String,
}




/// Name of trading platform.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlatformEnum {
    Mt5,
}

impl PlatformEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Mt5 => "mt5",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "mt5" => Some(Self::Mt5),
            _ => None,
        }
    }
}
/// Must be `1`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TradingPlatformInvestorPasswordResetEnum {
    Value1 = 1,
}

impl TradingPlatformInvestorPasswordResetEnum {
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
