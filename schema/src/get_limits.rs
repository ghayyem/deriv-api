
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/get_limits/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 
use serde_json::Value;
use std::collections::HashMap;


// Import shared types from the *same* crate
use crate::lifetime_transfers::LifetimeTransfers; 
use crate::market_specific_value_item::MarketSpecificValueItem; 

// It's a struct
/// Trading limits of real account user
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetLimits {
    /// Maximum account cash balance\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub account_balance: Option<f64>,
    /// Cumulative daily transfer limits\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub daily_cumulative_amount_transfers: Option<Value>,
    /// Daily transfers\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub daily_transfers: Option<Value>,
    /// Maximum daily turnover\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub daily_turnover: Option<f64>,
    /// Lifetime withdrawal limit\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub lifetime_limit: Option<f64>,
    /// Lifetime transfer limits. Only present when applicable to the current accout.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub lifetime_transfers: Option<LifetimeTransfers>,
    /// Contains limitation information for each market.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub market_specific: Option<HashMap<String, Vec<MarketSpecificValueItem>>>,
    /// Number of days for num_of_days_limit withdrawal limit\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub num_of_days: Option<i64>,
    /// Withdrawal limit for num_of_days days\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub num_of_days_limit: Option<f64>,
    /// Maximum number of open positions\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub open_positions: Option<i64>,
    /// Maximum aggregate payouts on open positions\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub payout: Option<f64>,
    /// Maximum payout for each symbol based on different barrier types.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub payout_per_symbol: Option<Value>,
    /// Maximum aggregate payouts on open positions per symbol and contract type. This limit can be exceeded up to the overall payout limit if there is no prior open position.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub payout_per_symbol_and_contract_type: Option<f64>,
    /// Amount left to reach withdrawal limit\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub remainder: Option<f64>,
    /// Total withdrawal for num_of_days days\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub withdrawal_for_x_days_monetary: Option<f64>,
    /// Total withdrawal since inception\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub withdrawal_since_inception_monetary: Option<f64>,
}

