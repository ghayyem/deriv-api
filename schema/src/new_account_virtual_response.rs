
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/new_account_virtual/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Create virtual-money account
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct NewAccountVirtualResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// New virtual-money account details
    #[serde(rename = "new_account_virtual", skip_serializing_if = "Option::is_none")]
    pub new_account_virtual: NewAccountVirtual,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// New virtual-money account details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct NewAccountVirtual {
    /// Account balance
    #[serde(rename = "balance")]
    pub balance: f64,
    /// ID of the new virtual-money account
    #[serde(rename = "client_id")]
    pub client_id: String,
    /// Account currency
    #[serde(rename = "currency")]
    pub currency: String,
    /// Currency type against the currency
    #[serde(rename = "currency_type", skip_serializing_if = "Option::is_none")]
    pub currency_type: String,
    /// Email of the new virtual-money account
    #[serde(rename = "email")]
    pub email: String,
    /// Oauth token for the client's login session (so that the user may be logged in immediately)
    #[serde(rename = "oauth_token")]
    pub oauth_token: String,
    /// Refresh token to perform PTA, only for the first ever created account
    #[serde(rename = "refresh_token", skip_serializing_if = "Option::is_none")]
    pub refresh_token: String,
    /// Account type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type: TypeEnum,
}






