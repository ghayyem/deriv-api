
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_country_list/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 
use serde_json::Value;
use std::collections::HashMap;


// Import shared types from the *same* crate
use crate::cross_border_ads_enabled_enum::CrossBorderAdsEnabledEnum; 
use crate::fixed_rate_adverts_enum::FixedRateAdvertsEnum; 
use crate::float_rate_adverts_enum::FloatRateAdvertsEnum; 

// It's a struct
/// Country code identifier.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct P2pCountryListValue {
    /// Display name of country.\n
    // Correct serde attribute construction - Use helper
    
    pub country_name: String,
    /// When 1, users in this country may place orders on ads in other countries.\n
    // Correct serde attribute construction - Use helper
    
    pub cross_border_ads_enabled: CrossBorderAdsEnabledEnum,
    /// Availability of fixed rate adverts.\n
    // Correct serde attribute construction - Use helper
    
    pub fixed_rate_adverts: FixedRateAdvertsEnum,
    /// Availability of floating rate adverts.\n
    // Correct serde attribute construction - Use helper
    
    pub float_rate_adverts: FloatRateAdvertsEnum,
    /// Maximum rate offset for floating rate adverts.\n
    // Correct serde attribute construction - Use helper
    
    pub float_rate_offset_limit: f64,
    /// Local currency of the country.\n
    // Correct serde attribute construction - Use helper
    
    pub local_currency: String,
    /// Field 'payment_methods' mapped to Value due to complexity/potential issues.\n
    // Correct serde attribute construction - Use helper
    
    pub payment_methods: Value,
}

