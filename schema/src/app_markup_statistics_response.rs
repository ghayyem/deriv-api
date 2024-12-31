
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/app_markup_statistics/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Per application reporting of app_markup
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct AppMarkupStatisticsResponse {
    /// App Markup transaction statistics
    #[serde(rename = "app_markup_statistics", skip_serializing_if = "Option::is_none")]
    pub app_markup_statistics: AppMarkupStatistics,
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



/// App Markup transaction statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct AppMarkupStatistics {
    /// Array of summed app markups grouped by app_id
    #[serde(rename = "breakdown", skip_serializing_if = "Option::is_none")]
    pub breakdown: Vec<Breakdownitem>,
    /// The sum of markup the client paid in USD
    #[serde(rename = "total_app_markup_usd", skip_serializing_if = "Option::is_none")]
    pub total_app_markup_usd: f64,
    /// The total count of transactions
    #[serde(rename = "total_transactions_count", skip_serializing_if = "Option::is_none")]
    pub total_transactions_count: f64,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Breakdownitem {
    /// ID of the application where this contract was purchased.
    #[serde(rename = "app_id", skip_serializing_if = "Option::is_none")]
    pub app_id: i64,
    /// The sum of markup the client paid in USD
    #[serde(rename = "app_markup_usd", skip_serializing_if = "Option::is_none")]
    pub app_markup_usd: f64,
    /// The sum of markup the client paid in developer's currency
    #[serde(rename = "app_markup_value", skip_serializing_if = "Option::is_none")]
    pub app_markup_value: f64,
    /// Currency code of the app developer
    #[serde(rename = "dev_currcode", skip_serializing_if = "Option::is_none")]
    pub dev_currcode: String,
    /// The count of app transactions
    #[serde(rename = "transactions_count", skip_serializing_if = "Option::is_none")]
    pub transactions_count: f64,
}








