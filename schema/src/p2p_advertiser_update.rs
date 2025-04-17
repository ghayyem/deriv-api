
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_advertiser_update/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 
use serde_json::Value;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

// Import shared types from the *same* crate
use crate::show_name_enum::ShowNameEnum; 
use crate::is_listed_enum::IsListedEnum; 
use crate::is_approved_enum::IsApprovedEnum; 
use crate::is_schedule_available_enum::IsScheduleAvailableEnum; 
use crate::is_online_enum::IsOnlineEnum; 
use crate::upgradable_daily_limits::UpgradableDailyLimits; 
use crate::full_verification_enum::FullVerificationEnum; 
use crate::basic_verification_enum::BasicVerificationEnum; 
use crate::block_trade::BlockTrade; 

// It's a struct
/// P2P advertiser information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct P2pAdvertiserUpdate {
    /// Number of active fixed rate adverts belonging to the advertiser.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub active_fixed_ads: Option<i64>,
    /// Number of active floating rate adverts belonging to the advertiser.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub active_float_ads: Option<i64>,
    /// Average difference of advert rate compared to the market rate over the past 30 days.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub advert_rates: Option<f64>,
    /// Amount of funds available to sell on P2P. May be less than account balance according to deposit methods used.\n
    // Correct serde attribute construction - Use helper
    
    pub balance_available: f64,
    /// Boolean value: 1 or 0, indicating whether the advertiser's identify has been verified.\n
    // Correct serde attribute construction - Use helper
    
    pub basic_verification: BasicVerificationEnum,
    /// Block trading limits, if block trading is allowed.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub block_trade: Option<BlockTrade>,
    /// The number of P2P users who have blocked this advertiser.\n
    // Correct serde attribute construction - Use helper
    
    pub blocked_by_count: i64,
    /// If a temporary bar was placed, this is the epoch time at which it will end.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub blocked_until: Option<DateTime<Utc>>,
    /// The percentage of completed orders out of total orders as a buyer within the past 30 days.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub buy_completion_rate: Option<f64>,
    /// Buy order volume in the past 30 days.\n
    // Correct serde attribute construction - Use helper
    
    pub buy_orders_amount: f64,
    /// The number of buy order completed within the past 30 days.\n
    // Correct serde attribute construction - Use helper
    
    pub buy_orders_count: i64,
    /// The average time in seconds taken to make payment as a buyer within the past 30 days.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub buy_time_avg: Option<i64>,
    /// The average time in seconds taken to cancel orders as a buyer within the past 30 days.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub cancel_time_avg: Option<i64>,
    /// The number of times the user may cancel orders before being temporarily blocked.\n
    // Correct serde attribute construction - Use helper
    
    pub cancels_remaining: i64,
    /// The token to be used for authenticating the client for chat.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub chat_token: Option<String>,
    /// The unique identifier for the chat user.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub chat_user_id: Option<String>,
    /// Advertiser's contact information.\n
    // Correct serde attribute construction - Use helper
    
    pub contact_info: String,
    /// The epoch time that the client became an advertiser.\n
    // Correct serde attribute construction - Use helper
    
    pub created_time: DateTime<Utc>,
    /// Total value of P2P buy transactions in the past 24 hours.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub daily_buy: Option<f64>,
    /// Maximum allowed value of P2P buy transactions in a 24 hour period.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub daily_buy_limit: Option<f64>,
    /// Total value of P2P sell transactions in the past 24 hours.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub daily_sell: Option<f64>,
    /// Maximum allowed value of P2P sell transactions in a 24 hour period.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub daily_sell_limit: Option<f64>,
    /// Default description that can be used every time an advert is created.\n
    // Correct serde attribute construction - Use helper
    
    pub default_advert_description: String,
    /// The advertiser's first name.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub first_name: Option<String>,
    /// Boolean value: 1 or 0, indicating whether the advertiser's address has been verified.\n
    // Correct serde attribute construction - Use helper
    
    pub full_verification: FullVerificationEnum,
    /// The advertiser's identification number.\n
    // Correct serde attribute construction - Use helper
    
    pub id: String,
    /// The approval status of the advertiser.\n
    // Correct serde attribute construction - Use helper
    
    pub is_approved: IsApprovedEnum,
    /// Indicates if the advertiser's active adverts are listed. When `0`, adverts won't be listed regardless if they are active or not.\n
    // Correct serde attribute construction - Use helper
    
    pub is_listed: IsListedEnum,
    /// Indicates if the advertiser is currently online.\n
    // Correct serde attribute construction - Use helper
    
    pub is_online: IsOnlineEnum,
    /// Inidcates whether the advertiser's schedule allows P2P transactions at the current time.\n
    // Correct serde attribute construction - Use helper
    
    pub is_schedule_available: IsScheduleAvailableEnum,
    /// The advertiser's last name.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub last_name: Option<String>,
    /// Epoch of the latest time the advertiser was online, up to 6 months.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub last_online_time: Option<i64>,
    /// Maximum order amount for adverts.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub max_order_amount: Option<f64>,
    /// Sell ads will be hidden when your available balance or remaining daily sell limit falls beneath this value.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub min_balance: Option<f64>,
    /// Minimum order amount for adverts.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub min_order_amount: Option<f64>,
    /// The advertiser's displayed name.\n
    // Correct serde attribute construction - Use helper
    
    pub name: String,
    /// Number of different users the advertiser has traded with since registration.\n
    // Correct serde attribute construction - Use helper
    
    pub partner_count: i64,
    /// Advertiser's payment information.\n
    // Correct serde attribute construction - Use helper
    
    pub payment_info: String,
    /// Average rating of the advertiser, range is 1-5.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub rating_average: Option<f64>,
    /// Number of ratings given to the advertiser.\n
    // Correct serde attribute construction - Use helper
    
    pub rating_count: i64,
    /// Percentage of users who have recommended the advertiser.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub recommended_average: Option<f64>,
    /// Number of times the advertiser has been recommended.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub recommended_count: Option<i64>,
    /// The average time in seconds taken to release funds as a seller within the past 30 days.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub release_time_avg: Option<i64>,
    /// [Optional] Weekly availability schedule. Ads are visible and orders can be created only during available periods.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub schedule: Option<Vec<Value>>,
    /// The percentage of completed orders out of total orders as a seller within the past 30 days.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub sell_completion_rate: Option<f64>,
    /// Sell order volume in the past 30 days.\n
    // Correct serde attribute construction - Use helper
    
    pub sell_orders_amount: f64,
    /// The number of sell order orders completed within the past 30 days.\n
    // Correct serde attribute construction - Use helper
    
    pub sell_orders_count: i64,
    /// When `1`, the advertiser's real name will be displayed on to other users on adverts and orders.\n
    // Correct serde attribute construction - Use helper
    
    pub show_name: ShowNameEnum,
    /// The percentage of completed orders out of all orders within the past 30 days.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub total_completion_rate: Option<f64>,
    /// The total number of orders completed since advertiser registration.\n
    // Correct serde attribute construction - Use helper
    
    pub total_orders_count: i64,
    /// Total order volume since advertiser registration.\n
    // Correct serde attribute construction - Use helper
    
    pub total_turnover: String,
    /// New daily limits available.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub upgradable_daily_limits: Option<UpgradableDailyLimits>,
    /// Remaining withdrawal_limit of a non-fully authenticated advertiser.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub withdrawal_limit: Option<String>,
}

