
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/proposal/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 
use serde_json::Value;
use std::collections::HashMap;


// Import shared types from the *same* crate
use crate::contract_details::ContractDetails; 
use crate::validation_params::ValidationParams; 
use crate::limit_order::LimitOrder; 
use crate::cancellation::Cancellation; 

// It's a struct
/// Latest price and other details for a given contract
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Proposal {
    /// The ask price.\n
    // Correct serde attribute construction - Use helper
    
    pub ask_price: f64,
    /// [Only for vanilla options] The choices of predefined strike price for client to choose\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub barrier_choices: Option<Vec<Value>>,
    /// [Only for Turbos] The relative distance between current spot and the barrier.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub barrier_spot_distance: Option<String>,
    /// Contains information about contract cancellation option.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub cancellation: Option<Cancellation>,
    /// Commission changed in percentage (%).\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub commission: Option<f64>,
    /// Contains contract information.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub contract_details: Option<ContractDetails>,
    /// The end date of the contract.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub date_expiry: Option<i64>,
    /// The start date of the contract.\n
    // Correct serde attribute construction - Use helper
    
    pub date_start: i64,
    /// [Only for vanilla or turbos options] The implied number of contracts\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub display_number_of_contracts: Option<String>,
    /// Same as `ask_price`.\n
    // Correct serde attribute construction - Use helper
    
    pub display_value: f64,
    /// A per-connection unique identifier. Can be passed to the `forget` API call to unsubscribe.\n
    // Correct serde attribute construction - Use helper
    
    pub id: String,
    /// Contains limit order information. (Only applicable for contract with limit order).\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub limit_order: Option<LimitOrder>,
    /// Example: Win payout if Random 100 Index is strictly higher than entry spot at 15 minutes after contract start time.\n
    // Correct serde attribute construction - Use helper
    
    pub longcode: String,
    /// [Only for vanilla or turbos options] Maximum stakes allowed\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub max_stake: Option<f64>,
    /// [Only for vanilla or turbos options] Minimum stakes allowed\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub min_stake: Option<f64>,
    /// [Only for lookback trades] Multiplier applies when calculating the final payoff for each type of lookback. e.g. (Exit spot - Lowest historical price) * multiplier = Payout\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub multiplier: Option<f64>,
    /// The payout amount of the contract.\n
    // Correct serde attribute construction - Use helper
    
    pub payout: f64,
    /// [Only for Turbos] The choices of predefined payout per point for client to choose\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub payout_choices: Option<Vec<String>>,
    /// Spot value (if there are no Exchange data-feed licensing restrictions for the underlying symbol).\n
    // Correct serde attribute construction - Use helper
    
    pub spot: f64,
    /// The corresponding time of the spot value.\n
    // Correct serde attribute construction - Use helper
    
    pub spot_time: i64,
    /// Contains contract validation information.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub validation_params: Option<ValidationParams>,
}

