
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/new_partner_account/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Create real partner account - Receive
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct NewPartnerAccountResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// New real partner account details
    #[serde(rename = "new_partner_account", skip_serializing_if = "Option::is_none")]
    pub new_partner_account: NewPartnerAccount,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// New real partner account details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct NewPartnerAccount {
    /// Client ID of new real partner account
    #[serde(rename = "client_id")]
    pub client_id: String,
    /// Currency of an account
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: String,
    /// Currency type against the currency
    #[serde(rename = "currency_type", skip_serializing_if = "Option::is_none")]
    pub currency_type: String,
    /// Landing company full name
    #[serde(rename = "landing_company")]
    pub landing_company: String,
    /// Landing company shortcode
    #[serde(rename = "landing_company_shortcode", skip_serializing_if = "Option::is_none")]
    pub landing_company_shortcode: String,
    /// OAuth token for client's login session
    #[serde(rename = "oauth_token")]
    pub oauth_token: String,
}





