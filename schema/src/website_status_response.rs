
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/website_status/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Server status alongside general settings like call limits, currencies information, supported languages, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct WebsiteStatusResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// For subscription requests only.
    #[serde(rename = "subscription", skip_serializing_if = "Option::is_none")]
    pub subscription: Subscription,
    /// Server status and other information regarding general settings
    #[serde(rename = "website_status", skip_serializing_if = "Option::is_none")]
    pub website_status: WebsiteStatus,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Server status and other information regarding general settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct WebsiteStatus {
    /// Maximum number of API calls during specified period of time.
    #[serde(rename = "api_call_limits")]
    pub api_call_limits: ApiCallLimits,
    /// List of all available broker codes.
    #[serde(rename = "broker_codes", skip_serializing_if = "Option::is_none")]
    pub broker_codes: Vec<String>,
    /// Country code of connected IP
    #[serde(rename = "clients_country", skip_serializing_if = "Option::is_none")]
    pub clients_country: String,
    /// Available currencies and their information
    #[serde(rename = "currencies_config", flatten)]
    pub currencies_config: HashMap<String, CurrenciesConfigvalue>,
    /// Suspension status of Dxtrade/DerivX API calls
    #[serde(rename = "dxtrade_status", skip_serializing_if = "Option::is_none")]
    pub dxtrade_status: DxtradeStatus,
    /// Text for site status banner, contains problem description. shown only if set by the system.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: String,
    /// Suspension status of MT5 API calls
    #[serde(rename = "mt5_status", skip_serializing_if = "Option::is_none")]
    pub mt_5_status: Mt5Status,
    /// Peer-to-peer payment system settings.
    #[serde(rename = "p2p_config", skip_serializing_if = "Option::is_none")]
    pub p_2p_config: P2pConfig,
    /// Payments Agents system settings.
    #[serde(rename = "payment_agents", skip_serializing_if = "Option::is_none")]
    pub payment_agents: PaymentAgents,
    /// The current status of the website.
    #[serde(rename = "site_status", skip_serializing_if = "Option::is_none")]
    pub site_status: SiteStatusEnum,
    /// Provides codes for languages supported.
    #[serde(rename = "supported_languages", skip_serializing_if = "Option::is_none")]
    pub supported_languages: Vec<String>,
    /// Latest terms and conditions version.
    #[serde(rename = "terms_conditions_version", skip_serializing_if = "Option::is_none")]
    pub terms_conditions_version: String,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Maximum number of API calls during specified period of time.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct ApiCallLimits {
    /// Maximum subscription to proposal calls.
    #[serde(rename = "max_proposal_subscription")]
    pub max_proposal_subscription: MaxProposalSubscription,
    /// Maximum number of general requests allowed during specified period of time.
    #[serde(rename = "max_requestes_general")]
    pub max_requestes_general: MaxRequestesGeneral,
    /// Maximum number of outcome requests allowed during specified period of time.
    #[serde(rename = "max_requests_outcome")]
    pub max_requests_outcome: MaxRequestsOutcome,
    /// Maximum number of pricing requests allowed during specified period of time.
    #[serde(rename = "max_requests_pricing")]
    pub max_requests_pricing: MaxRequestsPricing,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Maximum subscription to proposal calls.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct MaxProposalSubscription {
    /// Describes which calls this limit applies to.
    #[serde(rename = "applies_to")]
    pub applies_to: String,
    /// Maximum number of allowed calls.
    #[serde(rename = "max")]
    pub max: f64,
}






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Maximum number of general requests allowed during specified period of time.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct MaxRequestesGeneral {
    /// Describes which calls this limit applies to.
    #[serde(rename = "applies_to")]
    pub applies_to: String,
    /// The maximum of allowed calls per hour.
    #[serde(rename = "hourly")]
    pub hourly: f64,
    /// The maximum of allowed calls per minute.
    #[serde(rename = "minutely")]
    pub minutely: f64,
}






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Maximum number of outcome requests allowed during specified period of time.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct MaxRequestsOutcome {
    /// Describes which calls this limit applies to.
    #[serde(rename = "applies_to")]
    pub applies_to: String,
    /// The maximum of allowed calls per hour.
    #[serde(rename = "hourly")]
    pub hourly: f64,
    /// The maximum of allowed calls per minute.
    #[serde(rename = "minutely")]
    pub minutely: f64,
}






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Maximum number of pricing requests allowed during specified period of time.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct MaxRequestsPricing {
    /// Describes which calls this limit applies to.
    #[serde(rename = "applies_to")]
    pub applies_to: String,
    /// The maximum of allowed calls per hour.
    #[serde(rename = "hourly")]
    pub hourly: f64,
    /// The maximum of allowed calls per minute.
    #[serde(rename = "minutely")]
    pub minutely: f64,
}








// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Suspension status of Dxtrade/DerivX API calls
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct DxtradeStatus {
    /// Suspension of Dxtrade/DerivX API calls on all servers.
    #[serde(rename = "all", skip_serializing_if = "Option::is_none")]
    pub all: i64,
    /// Suspension of Dxtrade/DerivX API calls on demo servers.
    #[serde(rename = "demo", skip_serializing_if = "Option::is_none")]
    pub demo: i64,
    /// Suspension of Dxtrade/DerivX API calls on real trading servers.
    #[serde(rename = "real", skip_serializing_if = "Option::is_none")]
    pub real: i64,
}






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Suspension status of MT5 API calls
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Mt5Status {
    /// Suspension of MT5 API calls on demo servers.
    #[serde(rename = "demo", skip_serializing_if = "Option::is_none")]
    pub demo: Vec<Value>,
    /// Suspension of MT5 API calls on real trading servers.
    #[serde(rename = "real", skip_serializing_if = "Option::is_none")]
    pub real: Vec<Value>,
}






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Peer-to-peer payment system settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct P2pConfig {
    /// Maximum number of active ads allowed by an advertiser per currency pair and advert type (buy or sell).
    #[serde(rename = "adverts_active_limit")]
    pub adverts_active_limit: i64,
    /// Adverts will be deactivated if no activity occurs within this period, in days.
    #[serde(rename = "adverts_archive_period", skip_serializing_if = "Option::is_none")]
    pub adverts_archive_period: i64,
    /// Block trading settings
    #[serde(rename = "block_trade")]
    pub block_trade: BlockTrade,
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
    /// Time allowed for order payment, in minutes after order creation.
    #[serde(rename = "order_payment_period")]
    pub order_payment_period: i64,
    /// Local P2P exchange rate which should be used instead of those obtained from the `exchange_rates` call.
    #[serde(rename = "override_exchange_rate", skip_serializing_if = "Option::is_none")]
    pub override_exchange_rate: String,
    /// Indicates if the payment methods feature is enabled.
    #[serde(rename = "payment_methods_enabled")]
    pub payment_methods_enabled: PaymentMethodsEnabledEnum,
    /// Time after successful order completion during which reviews can be created, in hours.
    #[serde(rename = "review_period")]
    pub review_period: f64,
    /// List of currencies for which P2P is available
    #[serde(rename = "supported_currencies")]
    pub supported_currencies: Vec<String>,
}






/// The current status of the website.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SiteStatusEnum {
    Up,
    Down,
    Updating,
}

impl SiteStatusEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Up => "up",
            Self::Down => "down",
            Self::Updating => "updating",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "up" => Some(Self::Up),
            "down" => Some(Self::Down),
            "updating" => Some(Self::Updating),
            _ => None,
        }
    }
}


