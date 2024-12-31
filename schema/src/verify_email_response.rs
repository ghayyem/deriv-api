
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/verify_email/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Verify Email Receive
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct VerifyEmailResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// 1 for success (secure code has been sent to the email address)
    #[serde(rename = "verify_email", skip_serializing_if = "Option::is_none")]
    pub verify_email: VerifyEmailEnum,
}




/// 1 for success (secure code has been sent to the email address)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerifyEmailEnum {
    Value0,
    Value1 = 1,
}

impl VerifyEmailEnum {
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
