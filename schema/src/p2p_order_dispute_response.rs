
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_order_dispute/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Result of the P2P order disputing.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct P2pOrderDisputeResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Details of the disputed order.
    #[serde(rename = "p2p_order_dispute", skip_serializing_if = "Option::is_none")]
    pub p_2p_order_dispute: P2pOrderDispute,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Details of the disputed order.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct P2pOrderDispute {
    /// The currency of order.
    #[serde(rename = "account_currency")]
    pub account_currency: String,
    /// Details of the advert for this order.
    #[serde(rename = "advert_details")]
    pub advert_details: AdvertDetails,
    /// Details of the advertiser for this order.
    #[serde(rename = "advertiser_details")]
    pub advertiser_details: AdvertiserDetails,
    /// The amount of the order.
    #[serde(rename = "amount")]
    pub amount: f64,
    /// The amount of the order, formatted to appropriate decimal places.
    #[serde(rename = "amount_display")]
    pub amount_display: String,
    /// The URL to be used to initialise the chat for this order.
    #[serde(rename = "chat_channel_url")]
    pub chat_channel_url: String,
    /// Details of the client who created the order.
    #[serde(rename = "client_details")]
    pub client_details: ClientDetails,
    /// Seller contact information.
    #[serde(rename = "contact_info")]
    pub contact_info: String,
    /// The epoch time of the order creation.
    #[serde(rename = "created_time")]
    pub created_time: i64,
    /// Details of the order dispute.
    #[serde(rename = "dispute_details")]
    pub dispute_details: DisputeDetails,
    /// The epoch time in which the order will be expired.
    #[serde(rename = "expiry_time")]
    pub expiry_time: i64,
    /// The unique identifier for this order.
    #[serde(rename = "id")]
    pub id: String,
    /// `1` if the order is created for the advert of the current client, otherwise `0`.
    #[serde(rename = "is_incoming")]
    pub is_incoming: IsIncomingEnum,
    /// `1` if a review can be given, otherwise `0`.
    #[serde(rename = "is_reviewable")]
    pub is_reviewable: IsReviewableEnum,
    /// `1` if the latest order changes have been seen by the current client, otherwise `0`.
    #[serde(rename = "is_seen")]
    pub is_seen: IsSeenEnum,
    /// Local currency for this order.
    #[serde(rename = "local_currency")]
    pub local_currency: String,
    /// Payment instructions.
    #[serde(rename = "payment_info")]
    pub payment_info: String,
    /// Cost in local currency.
    #[serde(rename = "price")]
    pub price: f64,
    /// Cost in local currency, formatted to appropriate decimal places.
    #[serde(rename = "price_display")]
    pub price_display: String,
    /// Conversion rate of the order.
    #[serde(rename = "rate")]
    pub rate: f64,
    /// Conversion rate of the order, formatted to appropriate decimal places.
    #[serde(rename = "rate_display")]
    pub rate_display: String,
    /// Current order status.
    #[serde(rename = "status")]
    pub status: StatusEnum,
    /// Whether this is a buy or a sell.
    #[serde(rename = "type")]
    pub type: TypeEnum,
    /// If blocked for too many failed verification attempts, the epoch time that the block will end.
    #[serde(rename = "verification_lockout_until", skip_serializing_if = "Option::is_none")]
    pub verification_lockout_until: i64,
    /// If a verification request has already been made, the epoch time that another verification request can be made.
    #[serde(rename = "verification_next_request", skip_serializing_if = "Option::is_none")]
    pub verification_next_request: i64,
    /// Indicates that the seller in the process of confirming the order.
    #[serde(rename = "verification_pending", skip_serializing_if = "Option::is_none")]
    pub verification_pending: VerificationPendingEnum,
    /// Epoch time that the current verification token will expire.
    #[serde(rename = "verification_token_expiry", skip_serializing_if = "Option::is_none")]
    pub verification_token_expiry: i64,
}




/// Indicates that the seller in the process of confirming the order.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationPendingEnum {
    Value0,
    Value1 = 1,
}

impl VerificationPendingEnum {
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


