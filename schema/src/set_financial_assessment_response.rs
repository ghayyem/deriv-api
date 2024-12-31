
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/set_financial_assessment/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Set Financial Assessment Receive
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct SetFinancialAssessmentResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// The financial assessment score assigned to the submitted financial assessment
    #[serde(rename = "set_financial_assessment", skip_serializing_if = "Option::is_none")]
    pub set_financial_assessment: SetFinancialAssessment,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// The financial assessment score assigned to the submitted financial assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct SetFinancialAssessment {
    /// CFD score based on answers
    #[serde(rename = "cfd_score", skip_serializing_if = "Option::is_none")]
    pub cfd_score: i64,
    /// Financial information score based on answers
    #[serde(rename = "financial_information_score", skip_serializing_if = "Option::is_none")]
    pub financial_information_score: i64,
    /// Financial Assessment score based on answers
    #[serde(rename = "total_score", skip_serializing_if = "Option::is_none")]
    pub total_score: i64,
    /// Trading experience score based on answers
    #[serde(rename = "trading_score", skip_serializing_if = "Option::is_none")]
    pub trading_score: i64,
}






