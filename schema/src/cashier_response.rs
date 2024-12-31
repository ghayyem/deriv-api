
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/cashier/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Cashier information for the specified type.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct CashierResponse {
    /// Possible error codes are:
/// - `ASK_TNC_APPROVAL`: API call `tnc_approval`
/// - `ASK_AUTHENTICATE`
/// - `ASK_UK_FUNDS_PROTECTION`: API call `tnc_approval`
/// - `ASK_CURRENCY`: API call `set_account_currency`
/// - `ASK_EMAIL_VERIFY`: API call `verify_email`
/// - `ASK_FIX_DETAILS`: API call `set_settings`
    #[serde(rename = "cashier", skip_serializing_if = "Option::is_none")]
    pub cashier: Value,
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




