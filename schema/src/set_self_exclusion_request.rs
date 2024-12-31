
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/set_self_exclusion/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Set Self-Exclusion (this call should be used in conjunction with `get_self_exclusion`)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct SetSelfExclusionRequest {
    /// [Optional] Exclude me from the website (for a minimum of 6 months, up to a maximum of 5 years). Note: uplifting this self-exclusion may require contacting the company.
    #[serde(rename = "exclude_until", skip_serializing_if = "Option::is_none")]
    pub exclude_until: Option<Value>,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// [Optional] 7-day limit on deposits.
    #[serde(rename = "max_30day_deposit", skip_serializing_if = "Option::is_none")]
    pub max_30day_deposit: Option<Value>,
    /// [Optional] 30-day limit on losses.
    #[serde(rename = "max_30day_losses", skip_serializing_if = "Option::is_none")]
    pub max_30day_losses: Option<Value>,
    /// [Optional] 30-day turnover limit.
    #[serde(rename = "max_30day_turnover", skip_serializing_if = "Option::is_none")]
    pub max_30day_turnover: Option<Value>,
    /// [Optional] 7-day limit on deposits.
    #[serde(rename = "max_7day_deposit", skip_serializing_if = "Option::is_none")]
    pub max_7day_deposit: Option<Value>,
    /// [Optional] 7-day limit on losses.
    #[serde(rename = "max_7day_losses", skip_serializing_if = "Option::is_none")]
    pub max_7day_losses: Option<Value>,
    /// [Optional] 7-day turnover limit.
    #[serde(rename = "max_7day_turnover", skip_serializing_if = "Option::is_none")]
    pub max_7day_turnover: Option<Value>,
    /// [Optional] Maximum account cash balance.
    #[serde(rename = "max_balance", skip_serializing_if = "Option::is_none")]
    pub max_balance: Option<Value>,
    /// [Optional] Daily deposit limit.
    #[serde(rename = "max_deposit", skip_serializing_if = "Option::is_none")]
    pub max_deposit: Option<Value>,
    /// [Optional] Daily limit on losses.
    #[serde(rename = "max_losses", skip_serializing_if = "Option::is_none")]
    pub max_losses: Option<Value>,
    /// [Optional] Maximum number of open positions.
    #[serde(rename = "max_open_bets", skip_serializing_if = "Option::is_none")]
    pub max_open_bets: Option<Value>,
    /// [Optional] Daily turnover limit.
    #[serde(rename = "max_turnover", skip_serializing_if = "Option::is_none")]
    pub max_turnover: Option<Value>,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// [Optional] Session duration limit, in minutes.
    #[serde(rename = "session_duration_limit", skip_serializing_if = "Option::is_none")]
    pub session_duration_limit: Option<Value>,
    /// Must be `1`
    #[serde(rename = "set_self_exclusion")]
    pub set_self_exclusion: SetSelfExclusionEnum,
    /// [Optional] Exclude me from the website (for up to 6 weeks). Requires time in epoch format. Note: unlike `exclude_until`, this self-exclusion will be lifted automatically at the expiry of the timeout period.
    #[serde(rename = "timeout_until", skip_serializing_if = "Option::is_none")]
    pub timeout_until: Option<Value>,
}




/// Must be `1`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SetSelfExclusionEnum {
    Value1 = 1,
}

impl SetSelfExclusionEnum {
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
