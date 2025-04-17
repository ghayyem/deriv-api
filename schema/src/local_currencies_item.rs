
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_settings/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::is_default_enum::IsDefaultEnum; 
use crate::is_floating_rate_ad_supported_enum::IsFloatingRateAdSupportedEnum; 
use crate::has_adverts_enum::HasAdvertsEnum; 

// It's a struct
/// Local currency details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LocalCurrenciesItem {
    /// Local currency name\n
    // Correct serde attribute construction - Use helper
    
    pub display_name: String,
    /// Indicates that there are adverts available for this currency.\n
    // Correct serde attribute construction - Use helper
    
    pub has_adverts: HasAdvertsEnum,
    /// Indicates that this is local currency for the current country.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub is_default: Option<IsDefaultEnum>,
    /// Indicates that floating rate adverts are available for this currency.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub is_floating_rate_ad_supported: Option<IsFloatingRateAdSupportedEnum>,
    /// Local currency symbol\n
    // Correct serde attribute construction - Use helper
    
    pub symbol: String,
}

