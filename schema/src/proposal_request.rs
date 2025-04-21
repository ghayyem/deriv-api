
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/proposal/send.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;

use chrono::{DateTime, Utc};

// Import required types from the *same* crate
use crate::contract_type::ContractType;
use crate::limit_order::LimitOrder;
use crate::barrier_range::BarrierRange;
use crate::duration_unit::DurationUnit;

/// Gets latest price for a specific contract.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProposalRequest {
    /// [Optional] Proposed contract payout or stake, or multiplier (for lookbacks).\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub amount: Option<f64>,
    /// [Optional] Barrier for the contract (or last digit prediction for digit contracts). Contracts less than 24 hours in duration would need a relative barrier (barriers which need +/-), where entry spot would be adjusted accordingly with that amount to define a barrier, except for Synthetic Indices as they support both relative and absolute barriers. Not needed for lookbacks.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub barrier: Option<String>,
    /// [Optional] Low barrier for the contract (for contracts with two barriers). Contracts less than 24 hours in duration would need a relative barrier (barriers which need +/-), where entry spot would be adjusted accordingly with that amount to define a barrier, except for Synthetic Indices as they support both relative and absolute barriers. Not needed for lookbacks.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub barrier2: Option<String>,
    /// [Optional] Barrier range for callputspread.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub barrier_range: Option<BarrierRange>,
    /// [Optional] Indicates type of the `amount`.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub basis: Option<String>,
    /// Cancellation duration option (only for `MULTUP` and `MULTDOWN` contracts).\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub cancellation: Option<String>,
    /// The proposed contract type\n
    // Correct serde attribute construction - Use helper
    
    pub contract_type: ContractType,
    /// This can only be the account-holder's currency (obtained from `payout_currencies` call).\n
    // Correct serde attribute construction - Use helper
    
    pub currency: String,
    /// [Optional] Epoch value of the expiry time of the contract. Either date_expiry or duration is required.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub date_expiry: Option<String>,
    /// [Optional] Indicates epoch value of the starting time of the contract. If left empty, the start time of the contract is now.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub date_start: Option<DateTime<Utc>>,
    /// [Optional] Duration quantity. Either date_expiry or duration is required.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub duration: Option<i64>,
    /// [Optional] Duration unit - `s`: seconds, `m`: minutes, `h`: hours, `d`: days, `t`: ticks.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub duration_unit: Option<DurationUnit>,
    /// [Optional] Growth rate of an accumulator contract.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub growth_rate: Option<f64>,
    /// Add an order to close the contract once the order condition is met (only for `MULTUP` and `MULTDOWN` and 'ACCU' contracts). Supported orders: `take_profit`, `stop_loss`.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub limit_order: Option<LimitOrder>,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub loginid: Option<String>,
    /// [Optional] The multiplier for non-binary options. E.g. lookbacks.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub multiplier: Option<f64>,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub passthrough: Option<Value>,
    /// [Optional] Clients can provide payout_per_point directly, and the barrier will be calculated based on this payout_per_point value.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub payout_per_point: Option<String>,
    /// [Optional] The product type.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub product_type: Option<String>,
    /// Field 'proposal' mapped to Value due to complexity/potential issues.\n
    // Correct serde attribute construction - Use helper
    
    pub proposal: Value,
    /// [Optional] Used to map request to response.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub req_id: Option<i64>,
    /// [Optional] The tick that is predicted to have the highest/lowest value - for `TICKHIGH` and `TICKLOW` contracts.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub selected_tick: Option<String>,
    /// [Optional] 1 - to initiate a realtime stream of prices. Note that tick trades (without a user-defined barrier), digit trades and less than 24 hours at-the-money contracts for the following underlying symbols are not streamed: `R_10`, `R_25`, `R_50`, `R_75`, `R_100`, `RDBULL`, `RDBEAR` (this is because their price is constant).\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub subscribe: Option<String>,
    /// The short symbol name (obtained from `active_symbols` call).\n
    // Correct serde attribute construction - Use helper
    
    pub symbol: String,
    /// [Only for Snowball] The trade risk profile for the Snowball contract. Higher risk profile offers higher coupon rate at the expense of higher probability of breaching caution price\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub trade_risk_profile: Option<String>,
    /// [Optional] Required only for multi-barrier trading. Defines the epoch value of the trading period start time.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub trading_period_start: Option<DateTime<Utc>>,
}

