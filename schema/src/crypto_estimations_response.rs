
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/crypto_estimations/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Latest cryptocurrency estimations.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct CryptoEstimationsResponse {
    /// Cryptocurrency estimations. E.g. Withdrawal fee estimations.
    #[serde(rename = "crypto_estimations", skip_serializing_if = "Option::is_none", flatten)]
    pub crypto_estimations: HashMap<String, CryptoEstimationsvalue>,
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// For subscription requests only.
    #[serde(rename = "subscription", skip_serializing_if = "Option::is_none")]
    pub subscription: Subscription,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Cryptocurrency code
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct CryptoEstimationsvalue {
    /// Estimated fee for crypto withdrawal calculated based on the current network conditions.
    #[serde(rename = "withdrawal_fee", skip_serializing_if = "Option::is_none")]
    pub withdrawal_fee: WithdrawalFee,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;

use chrono::{DateTime, Utc};

/// Estimated fee for crypto withdrawal calculated based on the current network conditions.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct WithdrawalFee {
    /// Expiry time for the estimated fee in epoch.
    #[serde(rename = "expiry_time", skip_serializing_if = "Option::is_none")]
    pub expiry_time: i64,
    /// Unique identifier for the estimated fee which allows locking the fee for a client.
    #[serde(rename = "unique_id", skip_serializing_if = "Option::is_none")]
    pub unique_id: String,
    /// Value of current estimated fee.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: f64,
}








