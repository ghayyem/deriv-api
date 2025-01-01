
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_order_create/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// The information about the created P2P order.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct P2pOrderCreateResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Information of the creates P2P order.
    #[serde(rename = "p2p_order_create", skip_serializing_if = "Option::is_none")]
    pub p_2p_order_create: P2pOrderCreate,
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
use chrono::{DateTime, Utc};

/// Information of the creates P2P order.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct P2pOrderCreate {
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
    /// Supported payment methods. Comma separated list.
    #[serde(rename = "payment_method", skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<Value>,
    /// Details of available payment methods.
    #[serde(rename = "payment_method_details", skip_serializing_if = "Option::is_none", flatten)]
    pub payment_method_details: HashMap<String, PaymentMethodDetailsvalue>,
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
    /// The status of the created order.
    #[serde(rename = "status")]
    pub status: StatusEnum,
    /// Type of the order.
    #[serde(rename = "type")]
    pub type: TypeEnum,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Details of the advert for this order.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct AdvertDetails {
    /// Indicates if this is block trade advert or not.
    #[serde(rename = "block_trade")]
    pub block_trade: BlockTradeEnum,
    /// General information about the advert.
    #[serde(rename = "description")]
    pub description: String,
    /// The unique identifier for the advert.
    #[serde(rename = "id")]
    pub id: String,
    /// The payment method.
    #[serde(rename = "payment_method", skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<Value>,
    /// Type of the advert.
    #[serde(rename = "type")]
    pub type: TypeEnum,
}






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Details of the client who created the order.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct ClientDetails {
    /// The client's first name.
    #[serde(rename = "first_name", skip_serializing_if = "Option::is_none")]
    pub first_name: String,
    /// The client's unique P2P identifier.
    #[serde(rename = "id")]
    pub id: String,
    /// Indicates if the advertiser is currently online.
    #[serde(rename = "is_online")]
    pub is_online: IsOnlineEnum,
    /// The client's last name.
    #[serde(rename = "last_name", skip_serializing_if = "Option::is_none")]
    pub last_name: String,
    /// Epoch of the latest time the advertiser was online, up to 6 months.
    #[serde(rename = "last_online_time", skip_serializing_if = "Option::is_none")]
    pub last_online_time: Option<Value>,
    /// The client's account identifier.
    #[serde(rename = "loginid")]
    pub loginid: String,
    /// The client's displayed name.
    #[serde(rename = "name")]
    pub name: String,
}






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Details of the order dispute.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct DisputeDetails {
    /// The dispute reason
    #[serde(rename = "dispute_reason", skip_serializing_if = "Option::is_none")]
    pub dispute_reason: Option<Value>,
    /// The loginid of the client who's raising the dispute
    #[serde(rename = "disputer_loginid", skip_serializing_if = "Option::is_none")]
    pub disputer_loginid: Option<Value>,
}






/// `1` if the order is created for the advert of the current client, otherwise `0`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IsIncomingEnum {
    Value0,
    Value1 = 1,
}

impl IsIncomingEnum {
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
/// `1` if a review can be given, otherwise `0`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IsReviewableEnum {
    Value0,
    Value1 = 1,
}

impl IsReviewableEnum {
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
/// `1` if the latest order changes have been seen by the current client, otherwise `0`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IsSeenEnum {
    Value1 = 1,
    Value0,
}

impl IsSeenEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Value1 => "1",
            Self::Value0 => "0",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "1" => Some(Self::Value1),
            "0" => Some(Self::Value0),
            _ => None,
        }
    }
}


