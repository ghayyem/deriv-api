
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_advert_info/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 
use serde_json::Value;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

// Import shared types from the *same* crate
use crate::advertiser_details::AdvertiserDetails; 
use crate::is_visible_enum::IsVisibleEnum; 
use crate::deleted_enum::DeletedEnum; 
use crate::is_eligible_enum::IsEligibleEnum; 
use crate::is_active_enum::IsActiveEnum; 
use crate::visibility_status_item_enum::VisibilityStatusItemEnum; 
use crate::eligibility_status_item_enum::EligibilityStatusItemEnum; 
use crate::type_enum::TypeEnum; 
use crate::is_client_schedule_available_enum::IsClientScheduleAvailableEnum; 
use crate::payment_method_details_value::PaymentMethodDetailsValue; 
use crate::rate_type_enum::RateTypeEnum; 
use crate::block_trade_enum::BlockTradeEnum; 
use crate::counterparty_type_enum::CounterpartyTypeEnum; 

// It's a struct
/// P2P advert information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct P2pAdvertInfo {
    /// Currency for this advert. This is the system currency to be transferred between advertiser and client.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub account_currency: Option<String>,
    /// The number of active orders against this advert.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub active_orders: Option<i64>,
    /// Details of the advertiser for this advert.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub advertiser_details: Option<AdvertiserDetails>,
    /// The total amount specified in advert, in `account_currency`. It is only visible to the advert owner.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub amount: Option<f64>,
    /// The total amount specified in advert, in `account_currency`, formatted to appropriate decimal places. It is only visible to the advert owner.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub amount_display: Option<f64>,
    /// Indicates if this is block trade advert or not.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub block_trade: Option<BlockTradeEnum>,
    /// Advertiser contact information. Only applicable for 'sell adverts'.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub contact_info: Option<String>,
    /// Type of transaction from the opposite party's perspective.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub counterparty_type: Option<CounterpartyTypeEnum>,
    /// The target country code of the advert.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub country: Option<String>,
    /// The advert creation time in epoch.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub created_time: Option<DateTime<Utc>>,
    /// Days until automatic inactivation of this ad, if no activity occurs.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub days_until_archive: Option<i64>,
    /// Indicates that the advert has been deleted.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub deleted: Option<DeletedEnum>,
    /// General information about the advert.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub description: Option<String>,
    /// Conversion rate from account currency to local currency, using current market rate if applicable.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub effective_rate: Option<f64>,
    /// Conversion rate from account currency to local currency, using current market rate if applicable, formatted to appropriate decimal places.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub effective_rate_display: Option<String>,
    /// Reasons why the counterparty terms do not allow the current user to place orders against this advert. Possible values:\n/// - `completion_rate`: current user's 30 day completion rate is less than `min_completion_rate`.\n/// - `country`: current user's residence is not in `eligible_countries`.\n/// - `join_date`: current user registered on P2P less than `min_join_days` in the past.\n/// - `rating`: current user's average review rating is less than `min_rating`.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub eligibility_status: Option<Vec<EligibilityStatusItemEnum>>,
    /// 2 letter country codes. Counterparties who do not live in these countries are not allowed to place orders against this advert.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub eligible_countries: Option<Vec<String>>,
    /// The unique identifier for this advert.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub id: Option<String>,
    /// The activation status of the advert.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub is_active: Option<IsActiveEnum>,
    /// Inidcates whether the current user's schedule has availability from now until now + order_expiry_period.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub is_client_schedule_available: Option<IsClientScheduleAvailableEnum>,
    /// Indicates that the current user meets the counterparty terms for placing orders against this advert.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub is_eligible: Option<IsEligibleEnum>,
    /// Indicates that this advert will appear on the main advert list.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub is_visible: Option<IsVisibleEnum>,
    /// Local currency for this advert. This is the form of payment to be arranged directly between advertiser and client.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub local_currency: Option<String>,
    /// Maximum order amount specified in advert, in `account_currency`. It is only visible for advertisers.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub max_order_amount: Option<f64>,
    /// Maximum order amount specified in advert, in `account_currency`, formatted to appropriate decimal places. It is only visible to the advert owner.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub max_order_amount_display: Option<f64>,
    /// Maximum order amount at this time, in `account_currency`.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub max_order_amount_limit: Option<f64>,
    /// Maximum order amount at this time, in `account_currency`, formatted to appropriate decimal places.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub max_order_amount_limit_display: Option<f64>,
    /// Counterparties who have a 30 day completion rate less than this value are not allowed to place orders against this advert.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub min_completion_rate: Option<f64>,
    /// Counterparties who joined less than this number of days ago are not allowed to place orders against this advert.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub min_join_days: Option<i64>,
    /// Minimum order amount specified in advert, in `account_currency`. It is only visible for advertisers.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub min_order_amount: Option<f64>,
    /// Minimum order amount specified in advert, in `account_currency`, formatted to appropriate decimal places. It is only visible to the advert owner.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub min_order_amount_display: Option<f64>,
    /// Minimum order amount at this time, in `account_currency`.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub min_order_amount_limit: Option<f64>,
    /// Minimum order amount at this time, in `account_currency`, formatted to appropriate decimal places.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub min_order_amount_limit_display: Option<f64>,
    /// Counterparties who have an average rating less than this value are not allowed to place orders against this advert.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub min_rating: Option<f64>,
    /// Expiry period (seconds) for order created against this ad.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub order_expiry_period: Option<i64>,
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
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub rate: Option<f64>,
    /// Conversion rate formatted to appropriate decimal places.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub rate_display: Option<String>,
    /// Type of rate, fixed or floating.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub rate_type: Option<RateTypeEnum>,
    /// Amount currently available for orders, in `account_currency`. It is only visible for advertisers.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub remaining_amount: Option<f64>,
    /// Amount currently available for orders, in `account_currency`, formatted to appropriate decimal places. It is only visible to the advert owner.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub remaining_amount_display: Option<String>,
    /// Whether this is a buy or a sell.\n
    // Correct serde attribute construction - Use helper
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")] 
    pub r#type: Option<TypeEnum>,
    /// Reasons why an advert is not visible. Possible values:\n/// - `advert_fixed_rate_disabled`: fixed rate adverts are no longer available in the advert's country.\n/// - `advert_float_rate_disabled`: floating rate adverts are no longer available in the advert's country.\n/// - `advert_inactive`: the advert is set inactive.\n/// - `advert_max_limit`: the minimum order amount exceeds the system maximum order.\n/// - `advert_min_limit`: the maximum order amount is too small to be shown on the advert list.\n/// - `advert_remaining`: the remaining amount of the advert is below the minimum order.\n/// - `advert_no_payment_methods`: the advert has no valid payment methods.\n/// - `advertiser_ads_paused`: the advertiser has paused all adverts.\n/// - `advertiser_approval`: the advertiser's proof of identity is not verified.\n/// - `advertiser_balance`: the advertiser's P2P balance is less than the minimum order.\n/// - `advertiser_schedule`: the advertiser's schedule does not have availability between now and now + order_expiry_period.\n/// - `advertiser_block_trade_ineligible`: the advertiser is not currently eligible for block trading.\n/// - `advertiser_daily_limit`: the advertiser's remaining daily limit is less than the minimum order.\n/// - `advertiser_temp_ban`: the advertiser is temporarily banned from P2P.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub visibility_status: Option<Vec<VisibilityStatusItemEnum>>,
}

