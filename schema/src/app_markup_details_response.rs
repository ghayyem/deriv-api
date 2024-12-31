
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






