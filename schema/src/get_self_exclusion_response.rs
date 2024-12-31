
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/get_self_exclusion/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// A message with User Self-Exclusion
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct GetSelfExclusionResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// List of values set for self exclusion.
    #[serde(rename = "get_self_exclusion", skip_serializing_if = "Option::is_none")]
    pub get_self_exclusion: GetSelfExclusion,
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

use chrono::{DateTime, Utc};

/// List of values set for self exclusion.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct GetSelfExclusion {
    /// Exclude me from the website (for a minimum of 6 months, up to a maximum of 5 years). Note: uplifting this self-exclusion may require contacting the company.
    #[serde(rename = "exclude_until", skip_serializing_if = "Option::is_none")]
    pub exclude_until: String,
    /// 30-day limit on deposits
    #[serde(rename = "max_30day_deposit", skip_serializing_if = "Option::is_none")]
    pub max_30day_deposit: f64,
    /// 30-day limit on losses
    #[serde(rename = "max_30day_losses", skip_serializing_if = "Option::is_none")]
    pub max_30day_losses: f64,
    /// 30-day turnover limit
    #[serde(rename = "max_30day_turnover", skip_serializing_if = "Option::is_none")]
    pub max_30day_turnover: f64,
    /// 7-day limit on deposits
    #[serde(rename = "max_7day_deposit", skip_serializing_if = "Option::is_none")]
    pub max_7day_deposit: f64,
    /// 7-day limit on losses
    #[serde(rename = "max_7day_losses", skip_serializing_if = "Option::is_none")]
    pub max_7day_losses: f64,
    /// 7-day turnover limit
    #[serde(rename = "max_7day_turnover", skip_serializing_if = "Option::is_none")]
    pub max_7day_turnover: f64,
    /// Maximum account cash balance
    #[serde(rename = "max_balance", skip_serializing_if = "Option::is_none")]
    pub max_balance: f64,
    /// Daily limit on deposits
    #[serde(rename = "max_deposit", skip_serializing_if = "Option::is_none")]
    pub max_deposit: f64,
    /// Daily limit on losses
    #[serde(rename = "max_losses", skip_serializing_if = "Option::is_none")]
    pub max_losses: f64,
    /// Maximum number of open positions
    #[serde(rename = "max_open_bets", skip_serializing_if = "Option::is_none")]
    pub max_open_bets: i64,
    /// Daily turnover limit
    #[serde(rename = "max_turnover", skip_serializing_if = "Option::is_none")]
    pub max_turnover: f64,
    /// Session duration limit, in minutes
    #[serde(rename = "session_duration_limit", skip_serializing_if = "Option::is_none")]
    pub session_duration_limit: i64,
    /// Exclude me from the website (for up to 6 weeks). The time is in epoch format. Note: unlike `exclude_until`, this self-exclusion will be lifted automatically at the expiry of the timeout period.
    #[serde(rename = "timeout_until", skip_serializing_if = "Option::is_none")]
    pub timeout_until: i64,
}






