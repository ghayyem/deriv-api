
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/proposal/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::stop_loss::StopLoss; 
use crate::payout::Payout; 
use crate::take_profit::TakeProfit; 
use crate::stake::Stake; 

// It's a struct
/// Contains contract validation information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidationParams {
    /// [Only for Accumulators] Maximum payout for the contract.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub max_payout: Option<String>,
    /// [Only for Accumulators] Maximum ticks for the contract.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub max_ticks: Option<i64>,
    /// Contains information for minimum and maximum payout amount for the contract.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub payout: Option<Payout>,
    /// Contains information for minimum and maximum stake amount for the contract.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub stake: Option<Stake>,
    /// [Only for Multipliers] Contains information for minimum and maximum stop loss amount for the contract.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub stop_loss: Option<StopLoss>,
    /// Contains information for minimum and maximum take profit amount for the contract.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub take_profit: Option<TakeProfit>,
}

