
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_settings/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Information of the P2P settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct P2pSettingsResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Peer-to-peer payment system settings.
    #[serde(rename = "p2p_settings", skip_serializing_if = "Option::is_none")]
    pub p_2p_settings: P2pSettings,
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


/// Peer-to-peer payment system settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct P2pSettings {
    /// Maximum number of active ads allowed by an advertiser per currency pair and advert type (buy or sell).
    #[serde(rename = "adverts_active_limit")]
    pub adverts_active_limit: i64,
    /// Adverts will be deactivated if no activity occurs within this period, in days.
    #[serde(rename = "adverts_archive_period", skip_serializing_if = "Option::is_none")]
    pub adverts_archive_period: i64,
    /// Block trading settings
    #[serde(rename = "block_trade")]
    pub block_trade: BlockTrade,
    /// Advertiser schedule start and end times must be exact multiples of this value, unless it is zero.
    #[serde(rename = "business_hours_minutes_interval")]
    pub business_hours_minutes_interval: i64,
    /// A buyer will be blocked for this duration after exceeding the cancellation limit, in hours.
    #[serde(rename = "cancellation_block_duration")]
    pub cancellation_block_duration: i64,
    /// The period within which to count buyer cancellations, in hours.
    #[serde(rename = "cancellation_count_period")]
    pub cancellation_count_period: i64,
    /// A buyer may cancel an order within this period without negative consequences, in minutes after order creation.
    #[serde(rename = "cancellation_grace_period")]
    pub cancellation_grace_period: i64,
    /// A buyer will be temporarily barred after marking this number of cancellations within cancellation_period.
    #[serde(rename = "cancellation_limit")]
    pub cancellation_limit: i64,
    /// Recommended step values for choosing advert counterparty terms.
    #[serde(rename = "counterparty_term_steps", skip_serializing_if = "Option::is_none")]
    pub counterparty_term_steps: CounterpartyTermSteps,
    /// When 0, only exchanges in local currency are allowed for P2P advertiser.
    #[serde(rename = "cross_border_ads_enabled")]
    pub cross_border_ads_enabled: CrossBorderAdsEnabledEnum,
    /// When 1, the P2P service is unavailable.
    #[serde(rename = "disabled")]
    pub disabled: DisabledEnum,
    /// Indicates the availbility of certain backend features.
    #[serde(rename = "feature_level")]
    pub feature_level: i64,
    /// Availability of fixed rate adverts.
    #[serde(rename = "fixed_rate_adverts")]
    pub fixed_rate_adverts: FixedRateAdvertsEnum,
    /// Date on which fixed rate adverts will be deactivated.
    #[serde(rename = "fixed_rate_adverts_end_date", skip_serializing_if = "Option::is_none")]
    pub fixed_rate_adverts_end_date: String,
    /// Availability of floating rate adverts.
    #[serde(rename = "float_rate_adverts")]
    pub float_rate_adverts: FloatRateAdvertsEnum,
    /// Maximum rate offset for floating rate adverts.
    #[serde(rename = "float_rate_offset_limit")]
    pub float_rate_offset_limit: f64,
    /// Available local currencies for p2p_advert_list request.
    #[serde(rename = "local_currencies")]
    pub local_currencies: Vec<LocalCurrenciesitem>,
    /// Maximum amount of an advert, in USD.
    #[serde(rename = "maximum_advert_amount")]
    pub maximum_advert_amount: f64,
    /// Maximum amount of an order, in USD.
    #[serde(rename = "maximum_order_amount")]
    pub maximum_order_amount: f64,
    /// Maximum number of orders a user may create per day.
    #[serde(rename = "order_daily_limit")]
    pub order_daily_limit: i64,
    /// List of order expiry values available for adverts, in seconds.
    #[serde(rename = "order_expiry_options")]
    pub order_expiry_options: Vec<i64>,
    /// Time allowed for order payment, in minutes after order creation.
    #[serde(rename = "order_payment_period")]
    pub order_payment_period: i64,
    /// Local P2P exchange rate which should be used instead of those obtained from the `exchange_rates` call.
    #[serde(rename = "override_exchange_rate", skip_serializing_if = "Option::is_none")]
    pub override_exchange_rate: String,
    /// Indicates if the payment methods feature is enabled.
    #[serde(rename = "payment_methods_enabled")]
    pub payment_methods_enabled: PaymentMethodsEnabledEnum,
    /// Indicates if phone number verification is required to become a P2P advertiser.
    #[serde(rename = "pnv_required")]
    pub pnv_required: PnvRequiredEnum,
    /// Indicates if proof of address is required to become a P2P advertiser.
    #[serde(rename = "poa_required")]
    pub poa_required: PoaRequiredEnum,
    /// Time after successful order completion during which reviews can be created, in hours.
    #[serde(rename = "review_period")]
    pub review_period: f64,
    /// List of currencies for which P2P is available
    #[serde(rename = "supported_currencies")]
    pub supported_currencies: Vec<String>,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Recommended step values for choosing advert counterparty terms.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct CounterpartyTermSteps {
    /// Values for minimum 30 day completion rate.
    #[serde(rename = "completion_rate")]
    pub completion_rate: Vec<f64>,
    /// Values for minimum joined days.
    #[serde(rename = "join_days")]
    pub join_days: Vec<i64>,
    /// Values for minimum average rating.
    #[serde(rename = "rating")]
    pub rating: Vec<f64>,
}






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Local currency details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct LocalCurrenciesitem {
    /// Local currency name
    #[serde(rename = "display_name")]
    pub display_name: String,
    /// Indicates that there are adverts available for this currency.
    #[serde(rename = "has_adverts")]
    pub has_adverts: HasAdvertsEnum,
    /// Indicates that this is local currency for the current country.
    #[serde(rename = "is_default", skip_serializing_if = "Option::is_none")]
    pub is_default: IsDefaultEnum,
    /// Indicates that floating rate adverts are available for this currency.
    #[serde(rename = "is_floating_rate_ad_supported", skip_serializing_if = "Option::is_none")]
    pub is_floating_rate_ad_supported: IsFloatingRateAdSupportedEnum,
    /// Local currency symbol
    #[serde(rename = "symbol")]
    pub symbol: String,
}




/// Indicates that there are adverts available for this currency.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HasAdvertsEnum {
    Value0,
    Value1 = 1,
}

impl HasAdvertsEnum {
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
/// Indicates that this is local currency for the current country.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IsDefaultEnum {
    Value1 = 1,
}

impl IsDefaultEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Value1 => "1",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "1" => Some(Self::Value1),
            _ => None,
        }
    }
}
/// Indicates that floating rate adverts are available for this currency.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IsFloatingRateAdSupportedEnum {
    Value0,
    Value1 = 1,
}

impl IsFloatingRateAdSupportedEnum {
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


/// When 0, only exchanges in local currency are allowed for P2P advertiser.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CrossBorderAdsEnabledEnum {
    Value0,
    Value1 = 1,
}

impl CrossBorderAdsEnabledEnum {
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
/// When 1, the P2P service is unavailable.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisabledEnum {
    Value0,
    Value1 = 1,
}

impl DisabledEnum {
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
/// Availability of fixed rate adverts.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FixedRateAdvertsEnum {
    Disabled,
    Enabled,
    List_Only,
}

impl FixedRateAdvertsEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::Enabled => "enabled",
            Self::List_Only => "list_only",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "disabled" => Some(Self::Disabled),
            "enabled" => Some(Self::Enabled),
            "list_only" => Some(Self::List_Only),
            _ => None,
        }
    }
}
/// Availability of floating rate adverts.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FloatRateAdvertsEnum {
    Disabled,
    Enabled,
    List_Only,
}

impl FloatRateAdvertsEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::Enabled => "enabled",
            Self::List_Only => "list_only",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "disabled" => Some(Self::Disabled),
            "enabled" => Some(Self::Enabled),
            "list_only" => Some(Self::List_Only),
            _ => None,
        }
    }
}
/// Indicates if the payment methods feature is enabled.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodsEnabledEnum {
    Value0,
    Value1 = 1,
}

impl PaymentMethodsEnabledEnum {
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
/// Indicates if phone number verification is required to become a P2P advertiser.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PnvRequiredEnum {
    Value0,
    Value1 = 1,
}

impl PnvRequiredEnum {
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
/// Indicates if proof of address is required to become a P2P advertiser.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PoaRequiredEnum {
    Value0,
    Value1 = 1,
}

impl PoaRequiredEnum {
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


