
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/trading_platform_password_reset/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Reset the password of a Trading Platform Account
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct TradingPlatformPasswordResetRequest {
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// New password of the account. For validation (Accepts any printable ASCII character. DerivX: Must be within 8-25 characters, include numbers, lowercase, uppercase letters. Must not be the same as the user's email address). Accepts any printable ASCII character. MT5: Must be within 8-16 characters, include numbers, lowercase, uppercase letters and special characters. Must not be the same as the user's email address.
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
    #[serde(rename = "trading_platform_password_reset")]
    pub trading_platform_password_reset: TradingPlatformPasswordResetEnum,
    /// Email verification code (received from a `verify_email` call, which must be done first)
    #[serde(rename = "verification_code")]
    pub verification_code: String,
}




