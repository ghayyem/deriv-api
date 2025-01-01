
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_advert_create/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Returns the information of the created  P2P (Peer to Peer) advert.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct P2pAdvertCreateResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// The information of the created P2P advert.
    #[serde(rename = "p2p_advert_create", skip_serializing_if = "Option::is_none")]
    pub p_2p_advert_create: P2pAdvertCreate,
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

/// The information of the created P2P advert.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct P2pAdvertCreate {
    /// Currency for this advert. This is the system currency to be transferred between advertiser and client.
    #[serde(rename = "account_currency")]
    pub account_currency: String,
    /// The number of active orders against this advert.
    #[serde(rename = "active_orders")]
    pub active_orders: i64,
    /// Details of the advertiser for this advert.
    #[serde(rename = "advertiser_details")]
    pub advertiser_details: AdvertiserDetails,
    /// The total amount specified in advert, in `account_currency`.
    #[serde(rename = "amount")]
    pub amount: f64,
    /// The total amount specified in advert, in `account_currency`, formatted to appropriate decimal places.
    #[serde(rename = "amount_display")]
    pub amount_display: String,
    /// Indicates if this is block trade advert or not.
    #[serde(rename = "block_trade")]
    pub block_trade: BlockTradeEnum,
    /// Advertiser contact information. Only applicable for 'sell adverts'.
    #[serde(rename = "contact_info", skip_serializing_if = "Option::is_none")]
    pub contact_info: String,
    /// Type of transaction from the opposite party's perspective.
    #[serde(rename = "counterparty_type")]
    pub counterparty_type: CounterpartyTypeEnum,
    /// The target country code of the advert.
    #[serde(rename = "country")]
    pub country: String,
    /// The advert creation time in epoch.
    #[serde(rename = "created_time")]
    pub created_time: i64,
    /// General information about the advert.
    #[serde(rename = "description")]
    pub description: String,
    /// Conversion rate from account currency to local currency, using current market rate if applicable.
    #[serde(rename = "effective_rate", skip_serializing_if = "Option::is_none")]
    pub effective_rate: Option<Value>,
    /// Conversion rate from account currency to local currency, using current market rate if applicable, formatted to appropriate decimal places.
    #[serde(rename = "effective_rate_display", skip_serializing_if = "Option::is_none")]
    pub effective_rate_display: Option<Value>,
    /// 2 letter country codes. Counterparties who do not live in these countries are not allowed to place orders against this advert.
    #[serde(rename = "eligible_countries", skip_serializing_if = "Option::is_none")]
    pub eligible_countries: Vec<String>,
    /// The unique identifier for this advert.
    #[serde(rename = "id")]
    pub id: String,
    /// The activation status of the advert.
    #[serde(rename = "is_active")]
    pub is_active: IsActiveEnum,
    /// Indicates that this advert will appear on the main advert list.
    #[serde(rename = "is_visible")]
    pub is_visible: IsVisibleEnum,
    /// Local currency for this advert. This is the form of payment to be arranged directly between advertiser and client.
    #[serde(rename = "local_currency")]
    pub local_currency: String,
    /// Maximum order amount specified in advert, in `account_currency`.
    #[serde(rename = "max_order_amount")]
    pub max_order_amount: f64,
    /// Maximum order amount specified in advert, in `account_currency`, formatted to appropriate decimal places.
    #[serde(rename = "max_order_amount_display")]
    pub max_order_amount_display: String,
    /// Maximum order amount at this time, in `account_currency`.
    #[serde(rename = "max_order_amount_limit")]
    pub max_order_amount_limit: f64,
    /// Maximum order amount at this time, in `account_currency`, formatted to appropriate decimal places.
    #[serde(rename = "max_order_amount_limit_display")]
    pub max_order_amount_limit_display: String,
    /// Counterparties who have a 30 day completion rate less than this value are not allowed to place orders against this advert.
    #[serde(rename = "min_completion_rate", skip_serializing_if = "Option::is_none")]
    pub min_completion_rate: f64,
    /// Counterparties who joined less than this number of days ago are not allowed to place orders against this advert.
    #[serde(rename = "min_join_days", skip_serializing_if = "Option::is_none")]
    pub min_join_days: i64,
    /// Minimum order amount specified in advert, in `account_currency`.
    #[serde(rename = "min_order_amount")]
    pub min_order_amount: f64,
    /// Minimum order amount specified in advert, in `account_currency`, formatted to appropriate decimal places.
    #[serde(rename = "min_order_amount_display")]
    pub min_order_amount_display: String,
    /// Minimum order amount at this time, in `account_currency`.
    #[serde(rename = "min_order_amount_limit")]
    pub min_order_amount_limit: f64,
    /// Minimum order amount at this time, in `account_currency`, formatted to appropriate decimal places.
    #[serde(rename = "min_order_amount_limit_display")]
    pub min_order_amount_limit_display: String,
    /// Counterparties who have an average rating less than this value are not allowed to place orders against this advert.
    #[serde(rename = "min_rating", skip_serializing_if = "Option::is_none")]
    pub min_rating: f64,
    /// Expiry period (seconds) for order created against this ad.
    #[serde(rename = "order_expiry_period")]
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
    #[serde(rename = "rate")]
    pub rate: f64,
    /// Conversion rate formatted to appropriate decimal places.
    #[serde(rename = "rate_display")]
    pub rate_display: String,
    /// Type of rate, fixed or floating.
    #[serde(rename = "rate_type")]
    pub rate_type: RateTypeEnum,
    /// Amount currently available for orders, in `account_currency`.
    #[serde(rename = "remaining_amount")]
    pub remaining_amount: f64,
    /// Amount currently available for orders, in `account_currency`, formatted to appropriate decimal places.
    #[serde(rename = "remaining_amount_display")]
    pub remaining_amount_display: String,
    /// The type of advertisement in relation to the advertiser's Deriv account.
    #[serde(rename = "type")]
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




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Details of the advertiser for this advert.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct AdvertiserDetails {
    /// The total number of orders completed in the past 30 days.
    #[serde(rename = "completed_orders_count")]
    pub completed_orders_count: i64,
    /// The advertiser's first name.
    #[serde(rename = "first_name", skip_serializing_if = "Option::is_none")]
    pub first_name: String,
    /// The advertiser's unique identifier.
    #[serde(rename = "id")]
    pub id: String,
    /// Indicates if the advertiser is currently online.
    #[serde(rename = "is_online")]
    pub is_online: IsOnlineEnum,
    /// Inidcates whether the advertiser's schedule has availability between now and now + order_expiry_period.
    #[serde(rename = "is_schedule_available")]
    pub is_schedule_available: IsScheduleAvailableEnum,
    /// The advertiser's last name.
    #[serde(rename = "last_name", skip_serializing_if = "Option::is_none")]
    pub last_name: String,
    /// Epoch of the latest time the advertiser was online, up to 6 months.
    #[serde(rename = "last_online_time", skip_serializing_if = "Option::is_none")]
    pub last_online_time: Option<Value>,
    /// The advertiser's displayed name.
    #[serde(rename = "name")]
    pub name: String,
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
    /// The percentage of successfully completed orders made by or placed against the advertiser within the past 30 days.
    #[serde(rename = "total_completion_rate", skip_serializing_if = "Option::is_none")]
    pub total_completion_rate: Option<Value>,
}




/// Indicates if the advertiser is currently online.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IsOnlineEnum {
    Value0,
    Value1 = 1,
}

impl IsOnlineEnum {
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
/// Inidcates whether the advertiser's schedule has availability between now and now + order_expiry_period.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IsScheduleAvailableEnum {
    Value0,
    Value1 = 1,
}

impl IsScheduleAvailableEnum {
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


// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Unique identifier.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct PaymentMethodDetailsvalue {
    /// Display name of payment method.
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: String,
    /// Payment method fields.
    #[serde(rename = "fields", flatten)]
    pub fields: HashMap<String, Fieldsvalue>,
    /// Indicates whether method is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: IsEnabledEnum,
    /// Payment method identifier.
    #[serde(rename = "method")]
    pub method: String,
    /// Payment method type.
    #[serde(rename = "type")]
    pub type: TypeEnum,
    /// IDs of adverts that use this payment method.
    #[serde(rename = "used_by_adverts", skip_serializing_if = "Option::is_none")]
    pub used_by_adverts: Option<Value>,
    /// IDs of orders that use this payment method.
    #[serde(rename = "used_by_orders", skip_serializing_if = "Option::is_none")]
    pub used_by_orders: Option<Value>,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Field identifier.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Fieldsvalue {
    /// Display name of payment method field.
    #[serde(rename = "display_name")]
    pub display_name: String,
    /// Is field required or optional.
    #[serde(rename = "required")]
    pub required: i64,
    /// Field type.
    #[serde(rename = "type")]
    pub type: TypeEnum,
    /// Current value of payment method field.
    #[serde(rename = "value")]
    pub value: String,
}






/// Indicates whether method is enabled.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IsEnabledEnum {
    Value0,
    Value1 = 1,
}

impl IsEnabledEnum {
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


/// Type of transaction from the opposite party's perspective.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CounterpartyTypeEnum {
    Buy,
    Sell,
}

impl CounterpartyTypeEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Buy => "buy",
            Self::Sell => "sell",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "buy" => Some(Self::Buy),
            "sell" => Some(Self::Sell),
            _ => None,
        }
    }
}
/// The activation status of the advert.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IsActiveEnum {
    Value0,
    Value1 = 1,
}

impl IsActiveEnum {
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
/// Indicates that this advert will appear on the main advert list.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IsVisibleEnum {
    Value0,
    Value1 = 1,
}

impl IsVisibleEnum {
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
/// 
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisibilityStatusitemEnum {
    Advert_Fixed_Rate_Disabled,
    Advert_Float_Rate_Disabled,
    Advert_Inactive,
    Advert_Max_Limit,
    Advert_Min_Limit,
    Advert_Remaining,
    Advert_No_Payment_Methods,
    Advertiser_Ads_Paused,
    Advertiser_Approval,
    Advertiser_Balance,
    Advertiser_Block_Trade_Ineligible,
    Advertiser_Daily_Limit,
    Advertiser_Schedule,
    Advertiser_Temp_Ban,
}

impl VisibilityStatusitemEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Advert_Fixed_Rate_Disabled => "advert_fixed_rate_disabled",
            Self::Advert_Float_Rate_Disabled => "advert_float_rate_disabled",
            Self::Advert_Inactive => "advert_inactive",
            Self::Advert_Max_Limit => "advert_max_limit",
            Self::Advert_Min_Limit => "advert_min_limit",
            Self::Advert_Remaining => "advert_remaining",
            Self::Advert_No_Payment_Methods => "advert_no_payment_methods",
            Self::Advertiser_Ads_Paused => "advertiser_ads_paused",
            Self::Advertiser_Approval => "advertiser_approval",
            Self::Advertiser_Balance => "advertiser_balance",
            Self::Advertiser_Block_Trade_Ineligible => "advertiser_block_trade_ineligible",
            Self::Advertiser_Daily_Limit => "advertiser_daily_limit",
            Self::Advertiser_Schedule => "advertiser_schedule",
            Self::Advertiser_Temp_Ban => "advertiser_temp_ban",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "advert_fixed_rate_disabled" => Some(Self::Advert_Fixed_Rate_Disabled),
            "advert_float_rate_disabled" => Some(Self::Advert_Float_Rate_Disabled),
            "advert_inactive" => Some(Self::Advert_Inactive),
            "advert_max_limit" => Some(Self::Advert_Max_Limit),
            "advert_min_limit" => Some(Self::Advert_Min_Limit),
            "advert_remaining" => Some(Self::Advert_Remaining),
            "advert_no_payment_methods" => Some(Self::Advert_No_Payment_Methods),
            "advertiser_ads_paused" => Some(Self::Advertiser_Ads_Paused),
            "advertiser_approval" => Some(Self::Advertiser_Approval),
            "advertiser_balance" => Some(Self::Advertiser_Balance),
            "advertiser_block_trade_ineligible" => Some(Self::Advertiser_Block_Trade_Ineligible),
            "advertiser_daily_limit" => Some(Self::Advertiser_Daily_Limit),
            "advertiser_schedule" => Some(Self::Advertiser_Schedule),
            "advertiser_temp_ban" => Some(Self::Advertiser_Temp_Ban),
            _ => None,
        }
    }
}


