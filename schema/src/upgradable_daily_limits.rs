
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_advertiser_update/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::block_trade_enum::BlockTradeEnum; 

// It's a struct
/// New daily limits available.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpgradableDailyLimits {
    /// When `1`, upgrade will provide block trading.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub block_trade: Option<BlockTradeEnum>,
    /// Upgradable daily buy limit.\n
    // Correct serde attribute construction - Use helper
    
    pub max_daily_buy: String,
    /// Upgradable daily sell limit.\n
    // Correct serde attribute construction - Use helper
    
    pub max_daily_sell: String,
}

