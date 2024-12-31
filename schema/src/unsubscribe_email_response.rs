
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/unsubscribe_email/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// The result of the unsubscribe email request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct UnsubscribeEmailResponse {
    /// Customer User ID.
    #[serde(rename = "binary_user_id", skip_serializing_if = "Option::is_none")]
    pub binary_user_id: f64,
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// `1`: email notification unsubscribed sucssesfully, `0`: failed to unsubscribed email notification
    #[serde(rename = "email_unsubscribe_status", skip_serializing_if = "Option::is_none")]
    pub email_unsubscribe_status: EmailUnsubscribeStatusEnum,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
}




/// `1`: email notification unsubscribed sucssesfully, `0`: failed to unsubscribed email notification
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmailUnsubscribeStatusEnum {
    Value0,
    Value1 = 1,
}

impl EmailUnsubscribeStatusEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Value0 => "0",
            Self::Value1 => "1",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "0" => Some(Self::Value0),
            "1" => Some(Self::Value1),
            _ => None,
        }
    }
}
