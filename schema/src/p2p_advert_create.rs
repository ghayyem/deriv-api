
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_advert_create/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 
use serde_json::Value;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

// Import shared types from the *same* crate
use crate::visibility_status_item_enum::VisibilityStatusItemEnum; 
use crate::payment_method_details_value::PaymentMethodDetailsValue; 
use crate::advertiser_details::AdvertiserDetails; 
use crate::block_trade_enum::BlockTradeEnum; 
use crate::counterparty_type_enum::CounterpartyTypeEnum; 
use crate::rate_type_enum::RateTypeEnum; 
use crate::is_active_enum::IsActiveEnum; 
use crate::is_visible_enum::IsVisibleEnum; 
use crate::type_enum::TypeEnum; 

// It's a struct
/// The information of the created P2P advert.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct P2pAdvertCreate {
    /// Currency for this advert. This is the system currency to be transferred between advertiser and client.\n
    // Correct serde attribute construction - Use helper
    
    pub account_currency: String,
    /// The number of active orders against this advert.\n
    // Correct serde attribute construction - Use helper
    
    pub active_orders: i64,
    /// Details of the advertiser for this advert.\n
    // Correct serde attribute construction - Use helper
    
    pub advertiser_details: AdvertiserDetails,
    /// The total amount specified in advert, in `account_currency`.\n
    // Correct serde attribute construction - Use helper
    
    pub amount: f64,
    /// The total amount specified in advert, in `account_currency`, formatted to appropriate decimal places.\n
    // Correct serde attribute construction - Use helper
    
    pub amount_display: f64,
    /// Indicates if this is block trade advert or not.\n
    // Correct serde attribute construction - Use helper
    
    pub block_trade: BlockTradeEnum,
    /// Advertiser contact information. Only applicable for 'sell adverts'.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub contact_info: Option<String>,
    /// Type of transaction from the opposite party's perspective.\n
    // Correct serde attribute construction - Use helper
    
    pub counterparty_type: CounterpartyTypeEnum,
    /// The target country code of the advert.\n
    // Correct serde attribute construction - Use helper
    
    pub country: String,
    /// The advert creation time in epoch.\n
    // Correct serde attribute construction - Use helper
    
    pub created_time: DateTime<Utc>,
    /// General information about the advert.\n
    // Correct serde attribute construction - Use helper
    
    pub description: String,
    /// Conversion rate from account currency to local currency, using current market rate if applicable.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub effective_rate: Option<f64>,
    /// Conversion rate from account currency to local currency, using current market rate if applicable, formatted to appropriate decimal places.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub effective_rate_display: Option<String>,
    /// 2 letter country codes. Counterparties who do not live in these countries are not allowed to place orders against this advert.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub eligible_countries: Option<Vec<String>>,
    /// The unique identifier for this advert.\n
    // Correct serde attribute construction - Use helper
    
    pub id: String,
    /// The activation status of the advert.\n
    // Correct serde attribute construction - Use helper
    
    pub is_active: IsActiveEnum,
    /// Indicates that this advert will appear on the main advert list.\n
    // Correct serde attribute construction - Use helper
    
    pub is_visible: IsVisibleEnum,
    /// Local currency for this advert. This is the form of payment to be arranged directly between advertiser and client.\n
    // Correct serde attribute construction - Use helper
    
    pub local_currency: String,
    /// Maximum order amount specified in advert, in `account_currency`.\n
    // Correct serde attribute construction - Use helper
    
    pub max_order_amount: f64,
    /// Maximum order amount specified in advert, in `account_currency`, formatted to appropriate decimal places.\n
    // Correct serde attribute construction - Use helper
    
    pub max_order_amount_display: f64,
    /// Maximum order amount at this time, in `account_currency`.\n
    // Correct serde attribute construction - Use helper
    
    pub max_order_amount_limit: f64,
    /// Maximum order amount at this time, in `account_currency`, formatted to appropriate decimal places.\n
    // Correct serde attribute construction - Use helper
    
    pub max_order_amount_limit_display: f64,
    /// Counterparties who have a 30 day completion rate less than this value are not allowed to place orders against this advert.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub min_completion_rate: Option<f64>,
    /// Counterparties who joined less than this number of days ago are not allowed to place orders against this advert.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub min_join_days: Option<i64>,
    /// Minimum order amount specified in advert, in `account_currency`.\n
    // Correct serde attribute construction - Use helper
    
    pub min_order_amount: f64,
    /// Minimum order amount specified in advert, in `account_currency`, formatted to appropriate decimal places.\n
    // Correct serde attribute construction - Use helper
    
    pub min_order_amount_display: f64,
    /// Minimum order amount at this time, in `account_currency`.\n
    // Correct serde attribute construction - Use helper
    
    pub min_order_amount_limit: f64,
    /// Minimum order amount at this time, in `account_currency`, formatted to appropriate decimal places.\n
    // Correct serde attribute construction - Use helper
    
    pub min_order_amount_limit_display: f64,
    /// Counterparties who have an average rating less than this value are not allowed to place orders against this advert.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub min_rating: Option<f64>,
    /// Expiry period (seconds) for order created against this ad.\n
    // Correct serde attribute construction - Use helper
    
    pub order_expiry_period: i64,
    /// Payment instructions. Only applicable for 'sell adverts'.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub payment_info: Option<String>,
    /// Payment method name (deprecated).\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub payment_method: Option<String>,
    /// Details of available payment methods (sell adverts only).\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub payment_method_details: Option<HashMap<String, PaymentMethodDetailsValue>>,
    /// Names of supported payment methods.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub payment_method_names: Option<Vec<String>>,
    /// Cost of the advert in local currency.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub price: Option<f64>,
    /// Cost of the advert in local currency, formatted to appropriate decimal places.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub price_display: Option<String>,
    /// Conversion rate from advertiser's account currency to `local_currency`. An absolute rate value (fixed), or percentage offset from current market rate (floating).\n
    // Correct serde attribute construction - Use helper
    
    pub rate: f64,
    /// Conversion rate formatted to appropriate decimal places.\n
    // Correct serde attribute construction - Use helper
    
    pub rate_display: String,
    /// Type of rate, fixed or floating.\n
    // Correct serde attribute construction - Use helper
    
    pub rate_type: RateTypeEnum,
    /// Amount currently available for orders, in `account_currency`.\n
    // Correct serde attribute construction - Use helper
    
    pub remaining_amount: f64,
    /// Amount currently available for orders, in `account_currency`, formatted to appropriate decimal places.\n
    // Correct serde attribute construction - Use helper
    
    pub remaining_amount_display: String,
    /// The type of advertisement in relation to the advertiser's Deriv account.\n
    // Correct serde attribute construction - Use helper
    #[serde(rename = "type")] 
    pub r#type: TypeEnum,
    /// Reasons why an advert is not visible. Possible values:\n/// - `advert_fixed_rate_disabled`: fixed rate adverts are no longer available in the advert's country.\n/// - `advert_float_rate_disabled`: floating rate adverts are no longer available in the advert's country.\n/// - `advert_inactive`: the advert is set inactive.\n/// - `advert_max_limit`: the minimum order amount exceeds the system maximum order.\n/// - `advert_min_limit`: the maximum order amount is too small to be shown on the advert list.\n/// - `advert_remaining`: the remaining amount of the advert is below the minimum order.\n/// - `advert_no_payment_methods`: the advert has no valid payment methods.\n/// - `advertiser_ads_paused`: the advertiser has paused all adverts.\n/// - `advertiser_approval`: the advertiser's proof of identity is not verified.\n/// - `advertiser_balance`: the advertiser's P2P balance is less than the minimum order.\n/// - `advertiser_schedule`: the advertiser's schedule does not have availability between now and now + order_expiry_period.\n/// - `advertiser_block_trade_ineligible`: the advertiser is not currently eligible for block trading.\n/// - `advertiser_daily_limit`: the advertiser's remaining daily limit is less than the minimum order.\n/// - `advertiser_temp_ban`: the advertiser is temporarily banned from P2P.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub visibility_status: Option<Vec<VisibilityStatusItemEnum>>,
}

