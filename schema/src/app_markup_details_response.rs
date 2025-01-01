
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/app_markup_details/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Per transaction reporting of app_markup
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct AppMarkupDetailsResponse {
    /// App Markup transaction details
    #[serde(rename = "app_markup_details", skip_serializing_if = "Option::is_none")]
    pub app_markup_details: AppMarkupDetails,
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



/// App Markup transaction details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct AppMarkupDetails {
    /// Array of returned transactions
    #[serde(rename = "transactions", skip_serializing_if = "Option::is_none")]
    pub transactions: Vec<Transactionsitem>,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Transactionsitem {
    /// ID of the application where this contract was purchased.
    #[serde(rename = "app_id", skip_serializing_if = "Option::is_none")]
    pub app_id: i64,
    /// The markup the client paid in their currency
    #[serde(rename = "app_markup", skip_serializing_if = "Option::is_none")]
    pub app_markup: f64,
    /// The markup the client paid in USD
    #[serde(rename = "app_markup_usd", skip_serializing_if = "Option::is_none")]
    pub app_markup_usd: f64,
    /// The markup the client paid in the app developer's currency
    #[serde(rename = "app_markup_value", skip_serializing_if = "Option::is_none")]
    pub app_markup_value: f64,
    /// Currency code of the client
    #[serde(rename = "client_currcode", skip_serializing_if = "Option::is_none")]
    pub client_currcode: String,
    /// Login ID of the client
    #[serde(rename = "client_loginid", skip_serializing_if = "Option::is_none")]
    pub client_loginid: String,
    /// Currency code of the app developer
    #[serde(rename = "dev_currcode", skip_serializing_if = "Option::is_none")]
    pub dev_currcode: String,
    /// Login ID of the app developer
    #[serde(rename = "dev_loginid", skip_serializing_if = "Option::is_none")]
    pub dev_loginid: String,
    /// The transaction ID. Every contract (buy or sell) and every payment has a unique ID.
    #[serde(rename = "transaction_id", skip_serializing_if = "Option::is_none")]
    pub transaction_id: i64,
    /// The epoch value of purchase time of transaction
    #[serde(rename = "transaction_time", skip_serializing_if = "Option::is_none")]
    pub transaction_time: String,
}








