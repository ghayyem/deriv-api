
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/proposal/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::trade_risk_profile_enum::TradeRiskProfileEnum; 

// It's a struct
/// Contains contract information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ContractDetails {
    /// The markup amount charged on a client's stake amount\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub app_markup_amount: Option<f64>,
    /// Barrier of the contract.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub barrier: Option<String>,
    /// Absolute difference between high/low barrier and spot\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub barrier_spot_distance: Option<String>,
    /// The caution price for the Snowball contract. Breaching this price will reset the coupons accrued to 0.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub caution_price: Option<f64>,
    /// The coupon rate for the Snowball contract at which the stake will grow for each coupon accrued.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub coupon_rate: Option<String>,
    /// High barrier calculated based on current spot\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub high_barrier: Option<String>,
    /// Epoch of last tick considered for stat chart\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub last_tick_epoch: Option<i64>,
    /// Low barrier calculated based on current spot\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub low_barrier: Option<String>,
    /// Maximum payout that user can get out of a contract, contract will close automatically if payout reaches this number\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub maximum_payout: Option<f64>,
    /// Maximum stake that user can set to buy a contract\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub maximum_stake: Option<String>,
    /// Maximum duration that a contract can last, contract will close automatically after this number of ticks\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub maximum_ticks: Option<i64>,
    /// Minimum stake that user can set to buy a contract\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub minimum_stake: Option<String>,
    /// The maximum number of coupons available for the Snowball contract.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub num_of_coupons: Option<i64>,
    /// The profit price for the Snowball contract. Breaching this price will close the contract immediately.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub profit_price: Option<f64>,
    /// Tick size barrier for Accumulator contracts\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub tick_size_barrier: Option<f64>,
    /// [Accumulator] Tick size barrier in percentage, rounded off to 5 decimal places\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub tick_size_barrier_percentage: Option<String>,
    /// An array of numbers  to build a stat chart - each number represents the duration that spot stayed between barries\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub ticks_stayed_in: Option<Vec<i64>>,
    /// The trade risk profile for the Snowball contract.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub trade_risk_profile: Option<TradeRiskProfileEnum>,
}

