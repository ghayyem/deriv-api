
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_order_review/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Response for creating a P2P order review.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct P2pOrderReviewResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Details of the created order review.
    #[serde(rename = "p2p_order_review", skip_serializing_if = "Option::is_none")]
    pub p_2p_order_review: P2pOrderReview,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;

use chrono::{DateTime, Utc};

/// Details of the created order review.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct P2pOrderReview {
    /// The reviewed advertiser's identification number.
    #[serde(rename = "advertiser_id")]
    pub advertiser_id: String,
    /// The epoch time of the review.
    #[serde(rename = "created_time")]
    pub created_time: i64,
    /// The order identification number.
    #[serde(rename = "order_id")]
    pub order_id: String,
    /// Rating for the transaction, 1 to 5.
    #[serde(rename = "rating")]
    pub rating: i64,
    /// `1` if the advertiser is recommended, `0` if not recommended.
    #[serde(rename = "recommended", skip_serializing_if = "Option::is_none")]
    pub recommended: Option<RecommendedEnum>,
}






