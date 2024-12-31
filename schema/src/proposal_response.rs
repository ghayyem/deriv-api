
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/proposal/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Latest price and other details for a given contract
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct ProposalResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Latest price and other details for a given contract
    #[serde(rename = "proposal", skip_serializing_if = "Option::is_none")]
    pub proposal: Proposal,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// For subscription requests only.
    #[serde(rename = "subscription", skip_serializing_if = "Option::is_none")]
    pub subscription: Subscription,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Latest price and other details for a given contract
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Proposal {
    /// The ask price.
    #[serde(rename = "ask_price")]
    pub ask_price: f64,
    /// [Only for vanilla options] The choices of predefined strike price for client to choose
    #[serde(rename = "barrier_choices", skip_serializing_if = "Option::is_none")]
    pub barrier_choices: Vec<Value>,
    /// [Only for Turbos] The relative distance between current spot and the barrier.
    #[serde(rename = "barrier_spot_distance", skip_serializing_if = "Option::is_none")]
    pub barrier_spot_distance: String,
    /// Contains information about contract cancellation option.
    #[serde(rename = "cancellation", skip_serializing_if = "Option::is_none")]
    pub cancellation: Cancellation,
    /// Commission changed in percentage (%).
    #[serde(rename = "commission", skip_serializing_if = "Option::is_none")]
    pub commission: Option<Value>,
    /// Contains contract information.
    #[serde(rename = "contract_details", skip_serializing_if = "Option::is_none")]
    pub contract_details: ContractDetails,
    /// The end date of the contract.
    #[serde(rename = "date_expiry", skip_serializing_if = "Option::is_none")]
    pub date_expiry: i64,
    /// The start date of the contract.
    #[serde(rename = "date_start")]
    pub date_start: i64,
    /// [Only for vanilla or turbos options] The implied number of contracts
    #[serde(rename = "display_number_of_contracts", skip_serializing_if = "Option::is_none")]
    pub display_number_of_contracts: String,
    /// Same as `ask_price`.
    #[serde(rename = "display_value")]
    pub display_value: String,
    /// A per-connection unique identifier. Can be passed to the `forget` API call to unsubscribe.
    #[serde(rename = "id")]
    pub id: String,
    /// Contains limit order information. (Only applicable for contract with limit order).
    #[serde(rename = "limit_order", skip_serializing_if = "Option::is_none")]
    pub limit_order: LimitOrder,
    /// Example: Win payout if Random 100 Index is strictly higher than entry spot at 15 minutes after contract start time.
    #[serde(rename = "longcode")]
    pub longcode: String,
    /// [Only for vanilla or turbos options] Maximum stakes allowed
    #[serde(rename = "max_stake", skip_serializing_if = "Option::is_none")]
    pub max_stake: f64,
    /// [Only for vanilla or turbos options] Minimum stakes allowed
    #[serde(rename = "min_stake", skip_serializing_if = "Option::is_none")]
    pub min_stake: f64,
    /// [Only for lookback trades] Multiplier applies when calculating the final payoff for each type of lookback. e.g. (Exit spot - Lowest historical price) * multiplier = Payout
    #[serde(rename = "multiplier", skip_serializing_if = "Option::is_none")]
    pub multiplier: f64,
    /// The payout amount of the contract.
    #[serde(rename = "payout")]
    pub payout: f64,
    /// [Only for Turbos] The choices of predefined payout per point for client to choose
    #[serde(rename = "payout_choices", skip_serializing_if = "Option::is_none")]
    pub payout_choices: Vec<String>,
    /// Spot value (if there are no Exchange data-feed licensing restrictions for the underlying symbol).
    #[serde(rename = "spot")]
    pub spot: f64,
    /// The corresponding time of the spot value.
    #[serde(rename = "spot_time")]
    pub spot_time: i64,
    /// Contains contract validation information.
    #[serde(rename = "validation_params", skip_serializing_if = "Option::is_none")]
    pub validation_params: ValidationParams,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;

use chrono::{DateTime, Utc};

/// Contains information about contract cancellation option.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Cancellation {
    /// Ask price of contract cancellation option.
    #[serde(rename = "ask_price", skip_serializing_if = "Option::is_none")]
    pub ask_price: f64,
    /// Expiry time in epoch for contract cancellation option.
    #[serde(rename = "date_expiry", skip_serializing_if = "Option::is_none")]
    pub date_expiry: i64,
}






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Contains contract information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct ContractDetails {
    /// The markup amount charged on a client's stake amount
    #[serde(rename = "app_markup_amount", skip_serializing_if = "Option::is_none")]
    pub app_markup_amount: String,
    /// Barrier of the contract.
    #[serde(rename = "barrier", skip_serializing_if = "Option::is_none")]
    pub barrier: String,
    /// Absolute difference between high/low barrier and spot
    #[serde(rename = "barrier_spot_distance", skip_serializing_if = "Option::is_none")]
    pub barrier_spot_distance: String,
    /// High barrier calculated based on current spot
    #[serde(rename = "high_barrier", skip_serializing_if = "Option::is_none")]
    pub high_barrier: String,
    /// Epoch of last tick considered for stat chart
    #[serde(rename = "last_tick_epoch", skip_serializing_if = "Option::is_none")]
    pub last_tick_epoch: i64,
    /// Low barrier calculated based on current spot
    #[serde(rename = "low_barrier", skip_serializing_if = "Option::is_none")]
    pub low_barrier: String,
    /// Maximum payout that user can get out of a contract, contract will close automatically if payout reaches this number
    #[serde(rename = "maximum_payout", skip_serializing_if = "Option::is_none")]
    pub maximum_payout: f64,
    /// Maximum stake that user can set to buy a contract
    #[serde(rename = "maximum_stake", skip_serializing_if = "Option::is_none")]
    pub maximum_stake: String,
    /// Maximum duration that a contract can last, contract will close automatically after this number of ticks
    #[serde(rename = "maximum_ticks", skip_serializing_if = "Option::is_none")]
    pub maximum_ticks: i64,
    /// Minimum stake that user can set to buy a contract
    #[serde(rename = "minimum_stake", skip_serializing_if = "Option::is_none")]
    pub minimum_stake: String,
    /// Tick size barrier for Accumulator contracts
    #[serde(rename = "tick_size_barrier", skip_serializing_if = "Option::is_none")]
    pub tick_size_barrier: f64,
    /// [Accumulator] Tick size barrier in percentage, rounded off to 5 decimal places
    #[serde(rename = "tick_size_barrier_percentage", skip_serializing_if = "Option::is_none")]
    pub tick_size_barrier_percentage: String,
    /// An array of numbers  to build a stat chart - each number represents the duration that spot stayed between barries
    #[serde(rename = "ticks_stayed_in", skip_serializing_if = "Option::is_none")]
    pub ticks_stayed_in: Vec<i64>,
}






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Contains contract validation information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct ValidationParams {
    /// [Only for Accumulators] Maximum payout for the contract.
    #[serde(rename = "max_payout", skip_serializing_if = "Option::is_none")]
    pub max_payout: String,
    /// [Only for Accumulators] Maximum ticks for the contract.
    #[serde(rename = "max_ticks", skip_serializing_if = "Option::is_none")]
    pub max_ticks: i64,
    /// Contains information for minimum and maximum payout amount for the contract.
    #[serde(rename = "payout", skip_serializing_if = "Option::is_none")]
    pub payout: Payout,
    /// Contains information for minimum and maximum stake amount for the contract.
    #[serde(rename = "stake", skip_serializing_if = "Option::is_none")]
    pub stake: Stake,
    /// [Only for Multipliers] Contains information for minimum and maximum stop loss amount for the contract.
    #[serde(rename = "stop_loss", skip_serializing_if = "Option::is_none")]
    pub stop_loss: StopLoss,
    /// Contains information for minimum and maximum take profit amount for the contract.
    #[serde(rename = "take_profit", skip_serializing_if = "Option::is_none")]
    pub take_profit: TakeProfit,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Contains information for minimum and maximum payout amount for the contract.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Payout {
    /// Maximum payout allowed
    #[serde(rename = "max", skip_serializing_if = "Option::is_none")]
    pub max: String,
}






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Contains information for minimum and maximum stake amount for the contract.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Stake {
    /// Maximum stake allowed
    #[serde(rename = "max", skip_serializing_if = "Option::is_none")]
    pub max: String,
    /// Minimum stake allowed
    #[serde(rename = "min", skip_serializing_if = "Option::is_none")]
    pub min: String,
}










