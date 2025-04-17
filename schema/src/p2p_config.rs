
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/website_status/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::local_currencies_item::LocalCurrenciesItem; 
use crate::disabled_enum::DisabledEnum; 
use crate::payment_methods_enabled_enum::PaymentMethodsEnabledEnum; 
use crate::cross_border_ads_enabled_enum::CrossBorderAdsEnabledEnum; 
use crate::float_rate_adverts_enum::FloatRateAdvertsEnum; 
use crate::block_trade::BlockTrade; 
use crate::fixed_rate_adverts_enum::FixedRateAdvertsEnum; 

// It's a struct
/// Peer-to-peer payment system settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct P2pConfig {
    /// Maximum number of active ads allowed by an advertiser per currency pair and advert type (buy or sell).\n
    // Correct serde attribute construction - Use helper
    
    pub adverts_active_limit: i64,
    /// Adverts will be deactivated if no activity occurs within this period, in days.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub adverts_archive_period: Option<i64>,
    /// Block trading settings\n
    // Correct serde attribute construction - Use helper
    
    pub block_trade: BlockTrade,
    /// A buyer will be blocked for this duration after exceeding the cancellation limit, in hours.\n
    // Correct serde attribute construction - Use helper
    
    pub cancellation_block_duration: i64,
    /// The period within which to count buyer cancellations, in hours.\n
    // Correct serde attribute construction - Use helper
    
    pub cancellation_count_period: i64,
    /// A buyer may cancel an order within this period without negative consequences, in minutes after order creation.\n
    // Correct serde attribute construction - Use helper
    
    pub cancellation_grace_period: i64,
    /// A buyer will be temporarily barred after marking this number of cancellations within cancellation_period.\n
    // Correct serde attribute construction - Use helper
    
    pub cancellation_limit: i64,
    /// When 0, only exchanges in local currency are allowed for P2P advertiser.\n
    // Correct serde attribute construction - Use helper
    
    pub cross_border_ads_enabled: CrossBorderAdsEnabledEnum,
    /// When 1, the P2P service is unavailable.\n
    // Correct serde attribute construction - Use helper
    
    pub disabled: DisabledEnum,
    /// Indicates the availbility of certain backend features.\n
    // Correct serde attribute construction - Use helper
    
    pub feature_level: i64,
    /// Availability of fixed rate adverts.\n
    // Correct serde attribute construction - Use helper
    
    pub fixed_rate_adverts: FixedRateAdvertsEnum,
    /// Date on which fixed rate adverts will be deactivated.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub fixed_rate_adverts_end_date: Option<String>,
    /// Availability of floating rate adverts.\n
    // Correct serde attribute construction - Use helper
    
    pub float_rate_adverts: FloatRateAdvertsEnum,
    /// Maximum rate offset for floating rate adverts.\n
    // Correct serde attribute construction - Use helper
    
    pub float_rate_offset_limit: f64,
    /// Available local currencies for p2p_advert_list request.\n
    // Correct serde attribute construction - Use helper
    
    pub local_currencies: Vec<LocalCurrenciesItem>,
    /// Maximum amount of an advert, in USD.\n
    // Correct serde attribute construction - Use helper
    
    pub maximum_advert_amount: f64,
    /// Maximum amount of an order, in USD.\n
    // Correct serde attribute construction - Use helper
    
    pub maximum_order_amount: f64,
    /// Maximum number of orders a user may create per day.\n
    // Correct serde attribute construction - Use helper
    
    pub order_daily_limit: i64,
    /// Time allowed for order payment, in minutes after order creation.\n
    // Correct serde attribute construction - Use helper
    
    pub order_payment_period: i64,
    /// Local P2P exchange rate which should be used instead of those obtained from the `exchange_rates` call.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub override_exchange_rate: Option<String>,
    /// Indicates if the payment methods feature is enabled.\n
    // Correct serde attribute construction - Use helper
    
    pub payment_methods_enabled: PaymentMethodsEnabledEnum,
    /// Time after successful order completion during which reviews can be created, in hours.\n
    // Correct serde attribute construction - Use helper
    
    pub review_period: f64,
    /// List of currencies for which P2P is available\n
    // Correct serde attribute construction - Use helper
    
    pub supported_currencies: Vec<String>,
}

