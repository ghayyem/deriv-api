
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/copytrading_statistics/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// The statistics of the trader.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct CopytradingStatisticsResponse {
    /// Statistics of the trader
    #[serde(rename = "copytrading_statistics", skip_serializing_if = "Option::is_none")]
    pub copytrading_statistics: CopytradingStatistics,
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
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Statistics of the trader
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct CopytradingStatistics {
    /// This is the epoch the investor started trading.
    #[serde(rename = "active_since")]
    pub active_since: i64,
    /// Average seconds of keeping positions open.
    #[serde(rename = "avg_duration")]
    pub avg_duration: i64,
    /// Average loss of trades in percentage.
    #[serde(rename = "avg_loss")]
    pub avg_loss: f64,
    /// Average profitable trades in percentage.
    #[serde(rename = "avg_profit")]
    pub avg_profit: f64,
    /// Number of copiers for this trader.
    #[serde(rename = "copiers")]
    pub copiers: f64,
    /// Represents the net change in equity for a 12-month period.
    #[serde(rename = "last_12months_profitable_trades")]
    pub last_12months_profitable_trades: f64,
    /// Represents the net change in equity per month.
    #[serde(rename = "monthly_profitable_trades", flatten)]
    pub monthly_profitable_trades: HashMap<String, f64>,
    /// Trader performance probability.
    #[serde(rename = "performance_probability")]
    pub performance_probability: f64,
    /// Total number of trades for all time.
    #[serde(rename = "total_trades")]
    pub total_trades: i64,
    /// Represents the portfolio distribution by markets.
    #[serde(rename = "trades_breakdown", flatten)]
    pub trades_breakdown: HashMap<String, f64>,
    /// Number of profit trades in percentage.
    #[serde(rename = "trades_profitable")]
    pub trades_profitable: f64,
    /// Represents the net change in equity per year.
    #[serde(rename = "yearly_profitable_trades", skip_serializing_if = "Option::is_none", flatten)]
    pub yearly_profitable_trades: HashMap<String, f64>,
}






