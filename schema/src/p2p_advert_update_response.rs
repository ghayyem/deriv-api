
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_advert_update/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Returns information about the updated advert.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct P2pAdvertUpdateResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// P2P updated advert information.
    #[serde(rename = "p2p_advert_update", skip_serializing_if = "Option::is_none")]
    pub p_2p_advert_update: P2pAdvertUpdate,
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

/// P2P updated advert information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct P2pAdvertUpdate {
    /// Currency for this advert. This is the system currency to be transferred between advertiser and client.
    #[serde(rename = "account_currency", skip_serializing_if = "Option::is_none")]
    pub account_currency: String,
    /// The number of active orders against this advert.
    #[serde(rename = "active_orders", skip_serializing_if = "Option::is_none")]
    pub active_orders: i64,
    /// Details of the advertiser for this advert.
    #[serde(rename = "advertiser_details", skip_serializing_if = "Option::is_none")]
    pub advertiser_details: AdvertiserDetails,
    /// The total amount specified in advert, in `account_currency`.
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: f64,
    /// The total amount specified in advert, in `account_currency`, formatted to appropriate decimal places.
    #[serde(rename = "amount_display", skip_serializing_if = "Option::is_none")]
    pub amount_display: String,
    /// Indicates if this is block trade advert or not.
    #[serde(rename = "block_trade", skip_serializing_if = "Option::is_none")]
    pub block_trade: BlockTradeEnum,
    /// Advertiser contact information. Only applicable for 'sell adverts'.
    #[serde(rename = "contact_info", skip_serializing_if = "Option::is_none")]
    pub contact_info: String,
    /// Type of transaction from the opposite party's perspective.
    #[serde(rename = "counterparty_type", skip_serializing_if = "Option::is_none")]
    pub counterparty_type: CounterpartyTypeEnum,
    /// The target country code of the advert.
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: String,
    /// The advert creation time in epoch.
    #[serde(rename = "created_time", skip_serializing_if = "Option::is_none")]
    pub created_time: i64,
    /// Days until automatic inactivation of this ad, if no activity occurs.
    #[serde(rename = "days_until_archive", skip_serializing_if = "Option::is_none")]
    pub days_until_archive: i64,
    /// Indicates that the advert has been deleted.
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: DeletedEnum,
    /// General information about the advert.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: String,
    /// Conversion rate from account currency to local currency, using current market rate if applicable.
    #[serde(rename = "effective_rate", skip_serializing_if = "Option::is_none")]
    pub effective_rate: Option<Value>,
    /// Conversion rate from account currency to local currency, using current market rate if applicable, formatted to appropriate decimal places.
    #[serde(rename = "effective_rate_display", skip_serializing_if = "Option::is_none")]
    pub effective_rate_display: Option<Value>,
    /// 2 letter country codes. Counterparties who do not live in these countries are not allowed to place orders against this advert
    #[serde(rename = "eligible_countries", skip_serializing_if = "Option::is_none")]
    pub eligible_countries: Vec<String>,
    /// The unique identifier for this advert.
    #[serde(rename = "id")]
    pub id: String,
    /// The activation status of the advert.
    #[serde(rename = "is_active", skip_serializing_if = "Option::is_none")]
    pub is_active: IsActiveEnum,
    /// Indicates that this advert will appear on the main advert list.
    #[serde(rename = "is_visible", skip_serializing_if = "Option::is_none")]
    pub is_visible: IsVisibleEnum,
    /// Local currency for this advert. This is the form of payment to be arranged directly between advertiser and client.
    #[serde(rename = "local_currency", skip_serializing_if = "Option::is_none")]
    pub local_currency: String,
    /// Maximum order amount specified in advert, in `account_currency`.
    #[serde(rename = "max_order_amount", skip_serializing_if = "Option::is_none")]
    pub max_order_amount: f64,
    /// Maximum order amount specified in advert, in `account_currency`, formatted to appropriate decimal places.
    #[serde(rename = "max_order_amount_display", skip_serializing_if = "Option::is_none")]
    pub max_order_amount_display: String,
    /// Maximum order amount at this time, in `account_currency`.
    #[serde(rename = "max_order_amount_limit", skip_serializing_if = "Option::is_none")]
    pub max_order_amount_limit: f64,
    /// Maximum order amount at this time, in `account_currency`, formatted to appropriate decimal places.
    #[serde(rename = "max_order_amount_limit_display", skip_serializing_if = "Option::is_none")]
    pub max_order_amount_limit_display: String,
    /// Counterparties who have a 30 day completion rate less than this value are not allowed to place orders against this advert.
    #[serde(rename = "min_completion_rate", skip_serializing_if = "Option::is_none")]
    pub min_completion_rate: f64,
    /// Counterparties who joined less than this number of days ago are not allowed to place orders against this advert.
    #[serde(rename = "min_join_days", skip_serializing_if = "Option::is_none")]
    pub min_join_days: i64,
    /// Minimum order amount specified in advert, in `account_currency`. It is only visible to the advert owner.
    #[serde(rename = "min_order_amount", skip_serializing_if = "Option::is_none")]
    pub min_order_amount: f64,
    /// Minimum order amount specified in advert, in `account_currency`, formatted to appropriate decimal places.
    #[serde(rename = "min_order_amount_display", skip_serializing_if = "Option::is_none")]
    pub min_order_amount_display: String,
    /// Minimum order amount at this time, in `account_currency`.
    #[serde(rename = "min_order_amount_limit", skip_serializing_if = "Option::is_none")]
    pub min_order_amount_limit: f64,
    /// Minimum order amount at this time, in `account_currency`, formatted to appropriate decimal places.
    #[serde(rename = "min_order_amount_limit_display", skip_serializing_if = "Option::is_none")]
    pub min_order_amount_limit_display: String,
    /// Counterparties who have an average rating less than this value are not allowed to place orders against this advert.
    #[serde(rename = "min_rating", skip_serializing_if = "Option::is_none")]
    pub min_rating: f64,
    /// Expiry period (seconds) for order created against this ad.
    #[serde(rename = "order_expiry_period", skip_serializing_if = "Option::is_none")]
    pub order_expiry_period: i64,
    /// Payment instructions. Only applicable for 'sell adverts'.
    #[serde(rename = "payment_info", skip_serializing_if = "Option::is_none")]
    pub payment_info: String,
    /// Payment method name (deprecated).
    #[serde(rename = "payment_method", skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<Value>,
    /// Details of available payment methods (sell adverts only).
    #[serde(rename = "payment_method_details", skip_serializing_if = "Option::is_none", flatten)]
    pub payment_method_details: HashMap<String, PaymentMethodDetailsvalue>,
    /// Names of supported payment methods.
    #[serde(rename = "payment_method_names", skip_serializing_if = "Option::is_none")]
    pub payment_method_names: Vec<String>,
    /// Cost of the advert in local currency.
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<Value>,
    /// Cost of the advert in local currency, formatted to appropriate decimal places.
    #[serde(rename = "price_display", skip_serializing_if = "Option::is_none")]
    pub price_display: Option<Value>,
    /// Conversion rate from advertiser's account currency to `local_currency`. An absolute rate value (fixed), or percentage offset from current market rate (floating).
    #[serde(rename = "rate", skip_serializing_if = "Option::is_none")]
    pub rate: f64,
    /// Conversion rate formatted to appropriate decimal places.
    #[serde(rename = "rate_display", skip_serializing_if = "Option::is_none")]
    pub rate_display: String,
    /// Type of rate, fixed or floating.
    #[serde(rename = "rate_type", skip_serializing_if = "Option::is_none")]
    pub rate_type: RateTypeEnum,
    /// Amount currently available for orders, in `account_currency`.
    #[serde(rename = "remaining_amount", skip_serializing_if = "Option::is_none")]
    pub remaining_amount: f64,
    /// Amount currently available for orders, in `account_currency`, formatted to appropriate decimal places.
    #[serde(rename = "remaining_amount_display", skip_serializing_if = "Option::is_none")]
    pub remaining_amount_display: String,
    /// Whether this is a buy or a sell.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type: TypeEnum,
    /// Reasons why an advert is not visible. Possible values: 
/// - `advert_fixed_rate_disabled`: fixed rate adverts are no longer available in the advert's country.
/// - `advert_float_rate_disabled`: floating rate adverts are no longer available in the advert's country. 
/// - `advert_inactive`: the advert is set inactive. 
/// - `advert_max_limit`: the minimum order amount exceeds the system maximum order. 
/// - `advert_min_limit`: the maximum order amount is too small to be shown on the advert list. 
/// - `advert_remaining`: the remaining amount of the advert is below the minimum order. 
/// - `advert_no_payment_methods`: the advert has no valid payment methods. 
/// - `advertiser_ads_paused`: the advertiser has paused all adverts. 
/// - `advertiser_approval`: the advertiser's proof of identity is not verified. 
/// - `advertiser_balance`: the advertiser's P2P balance is less than the minimum order. 
/// - `advertiser_schedule`: the advertiser's schedule does not have availability between now and now + order_expiry_period. 
/// - `advertiser_block_trade_ineligible`: the advertiser is not currently eligible for block trading. 
/// - `advertiser_daily_limit`: the advertiser's remaining daily limit is less than the minimum order. 
/// - `advertiser_temp_ban`: the advertiser is temporarily banned from P2P.
    #[serde(rename = "visibility_status", skip_serializing_if = "Option::is_none")]
    pub visibility_status: Vec<VisibilityStatusitemEnum>,
}






