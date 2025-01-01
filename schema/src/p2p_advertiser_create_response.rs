
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_advertiser_create/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Returns information of the created advertiser.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct P2pAdvertiserCreateResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// P2P advertiser information.
    #[serde(rename = "p2p_advertiser_create", skip_serializing_if = "Option::is_none")]
    pub p_2p_advertiser_create: P2pAdvertiserCreate,
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

use chrono::{DateTime, Utc};

/// P2P advertiser information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct P2pAdvertiserCreate {
    /// Average difference of advert rate compared to the market rate over the past 30 days.
    #[serde(rename = "advert_rates", skip_serializing_if = "Option::is_none")]
    pub advert_rates: Option<Value>,
    /// Amount of funds available to sell on P2P. May be less than account balance according to deposit methods used.
    #[serde(rename = "balance_available")]
    pub balance_available: f64,
    /// Boolean value: 1 or 0, indicating whether the advertiser's identify has been verified.
    #[serde(rename = "basic_verification")]
    pub basic_verification: BasicVerificationEnum,
    /// The number of P2P users who have blocked this advertiser.
    #[serde(rename = "blocked_by_count")]
    pub blocked_by_count: i64,
    /// The percentage of completed orders out of total orders as a buyer within the past 30 days.
    #[serde(rename = "buy_completion_rate", skip_serializing_if = "Option::is_none")]
    pub buy_completion_rate: Option<Value>,
    /// Buy order volume in the past 30 days.
    #[serde(rename = "buy_orders_amount")]
    pub buy_orders_amount: String,
    /// The number of buy order completed within the past 30 days.
    #[serde(rename = "buy_orders_count")]
    pub buy_orders_count: i64,
    /// The average time in seconds taken to make payment as a buyer within the past 30 days.
    #[serde(rename = "buy_time_avg", skip_serializing_if = "Option::is_none")]
    pub buy_time_avg: Option<Value>,
    /// The average time in seconds taken to cancel orders as a buyer within the past 30 days.
    #[serde(rename = "cancel_time_avg", skip_serializing_if = "Option::is_none")]
    pub cancel_time_avg: Option<Value>,
    /// The number of times the user may cancel orders before being temporarily blocked.
    #[serde(rename = "cancels_remaining")]
    pub cancels_remaining: i64,
    /// The token to be used for authenticating the client for chat.
    #[serde(rename = "chat_token", skip_serializing_if = "Option::is_none")]
    pub chat_token: Option<Value>,
    /// The unique identifier for the chat user.
    #[serde(rename = "chat_user_id", skip_serializing_if = "Option::is_none")]
    pub chat_user_id: Option<Value>,
    /// Advertiser's contact information.
    #[serde(rename = "contact_info")]
    pub contact_info: String,
    /// The epoch time that the client became an advertiser.
    #[serde(rename = "created_time")]
    pub created_time: i64,
    /// Total value of P2P buy transactions in the past 24 hours.
    #[serde(rename = "daily_buy", skip_serializing_if = "Option::is_none")]
    pub daily_buy: String,
    /// Maximum allowed value of P2P buy transactions in a 24 hour period.
    #[serde(rename = "daily_buy_limit", skip_serializing_if = "Option::is_none")]
    pub daily_buy_limit: String,
    /// Total value of P2P sell transactions in the past 24 hours.
    #[serde(rename = "daily_sell", skip_serializing_if = "Option::is_none")]
    pub daily_sell: String,
    /// Maximum allowed value of P2P sell transactions in a 24 hour period.
    #[serde(rename = "daily_sell_limit", skip_serializing_if = "Option::is_none")]
    pub daily_sell_limit: String,
    /// Default description that can be used every time an advert is created.
    #[serde(rename = "default_advert_description")]
    pub default_advert_description: String,
    /// Boolean value: 1 or 0, indicating whether the advertiser's address has been verified.
    #[serde(rename = "full_verification")]
    pub full_verification: FullVerificationEnum,
    /// The advertiser's identification number.
    #[serde(rename = "id")]
    pub id: String,
    /// The approval status of the advertiser.
    #[serde(rename = "is_approved")]
    pub is_approved: IsApprovedEnum,
    /// Indicates if the advertiser's active adverts are listed. When `0`, adverts won't be listed regardless if they are active or not.
    #[serde(rename = "is_listed")]
    pub is_listed: IsListedEnum,
    /// Indicates if the advertiser is currently online.
    #[serde(rename = "is_online")]
    pub is_online: IsOnlineEnum,
    /// Inidcates whether the advertiser's schedule allows P2P transactions at the current time.
    #[serde(rename = "is_schedule_available")]
    pub is_schedule_available: IsScheduleAvailableEnum,
    /// Epoch of the latest time the advertiser was online, up to 6 months.
    #[serde(rename = "last_online_time", skip_serializing_if = "Option::is_none")]
    pub last_online_time: Option<Value>,
    /// Maximum order amount for adverts.
    #[serde(rename = "max_order_amount", skip_serializing_if = "Option::is_none")]
    pub max_order_amount: String,
    /// Sell ads will be hidden when your available balance or remaining daily sell limit falls beneath this value.
    #[serde(rename = "min_balance", skip_serializing_if = "Option::is_none")]
    pub min_balance: String,
    /// Minimum order amount for adverts.
    #[serde(rename = "min_order_amount", skip_serializing_if = "Option::is_none")]
    pub min_order_amount: String,
    /// The advertiser's displayed name.
    #[serde(rename = "name")]
    pub name: String,
    /// Number of different users the advertiser has traded with since registration.
    #[serde(rename = "partner_count")]
    pub partner_count: i64,
    /// Advertiser's payment information.
    #[serde(rename = "payment_info")]
    pub payment_info: String,
    /// Average rating of the advertiser, range is 1-5.
    #[serde(rename = "rating_average", skip_serializing_if = "Option::is_none")]
    pub rating_average: Option<Value>,
    /// Number of ratings given to the advertiser.
    #[serde(rename = "rating_count")]
    pub rating_count: i64,
    /// Percentage of users who have recommended the advertiser.
    #[serde(rename = "recommended_average", skip_serializing_if = "Option::is_none")]
    pub recommended_average: Option<Value>,
    /// Number of times the advertiser has been recommended.
    #[serde(rename = "recommended_count", skip_serializing_if = "Option::is_none")]
    pub recommended_count: Option<Value>,
    /// The average time in seconds taken to release funds as a seller within the past 30 days.
    #[serde(rename = "release_time_avg", skip_serializing_if = "Option::is_none")]
    pub release_time_avg: Option<Value>,
    /// [Optional] Weekly availability schedule. Ads are visible and orders can be created only during available periods.
    #[serde(rename = "schedule", skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Value>,
    /// The percentage of completed orders out of total orders as a seller within the past 30 days.
    #[serde(rename = "sell_completion_rate", skip_serializing_if = "Option::is_none")]
    pub sell_completion_rate: Option<Value>,
    /// Sell order volume in the past 30 days.
    #[serde(rename = "sell_orders_amount")]
    pub sell_orders_amount: String,
    /// The number of sell order orders completed within the past 30 days.
    #[serde(rename = "sell_orders_count")]
    pub sell_orders_count: i64,
    /// When `1`, the advertiser's real name will be displayed to other users on adverts and orders.
    #[serde(rename = "show_name")]
    pub show_name: ShowNameEnum,
    /// The percentage of completed orders out of all orders within the past 30 days.
    #[serde(rename = "total_completion_rate", skip_serializing_if = "Option::is_none")]
    pub total_completion_rate: Option<Value>,
    /// The total number of orders completed since advertiser registration.
    #[serde(rename = "total_orders_count")]
    pub total_orders_count: i64,
    /// Total order volume since advertiser registration.
    #[serde(rename = "total_turnover")]
    pub total_turnover: String,
    /// Remaining withdrawal_limit of a non-fully authenticated advertiser.
    #[serde(rename = "withdrawal_limit", skip_serializing_if = "Option::is_none")]
    pub withdrawal_limit: Option<Value>,
}




/// Boolean value: 1 or 0, indicating whether the advertiser's identify has been verified.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BasicVerificationEnum {
    Value1 = 1,
    Value0,
}

impl BasicVerificationEnum {
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
/// Boolean value: 1 or 0, indicating whether the advertiser's address has been verified.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FullVerificationEnum {
    Value1 = 1,
    Value0,
}

impl FullVerificationEnum {
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
/// The approval status of the advertiser.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IsApprovedEnum {
    Value0,
    Value1 = 1,
}

impl IsApprovedEnum {
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
/// Indicates if the advertiser's active adverts are listed. When `0`, adverts won't be listed regardless if they are active or not.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IsListedEnum {
    Value0,
    Value1 = 1,
}

impl IsListedEnum {
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
/// When `1`, the advertiser's real name will be displayed to other users on adverts and orders.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShowNameEnum {
    Value0,
    Value1 = 1,
}

impl ShowNameEnum {
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


