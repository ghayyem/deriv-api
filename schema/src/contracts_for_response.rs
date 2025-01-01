
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/contracts_for/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Get the list of currently available contracts
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct ContractsForResponse {
    /// List of available contracts. Note: if the user is authenticated, then only contracts allowed under his account will be returned.
    #[serde(rename = "contracts_for", skip_serializing_if = "Option::is_none")]
    pub contracts_for: ContractsFor,
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// List of available contracts. Note: if the user is authenticated, then only contracts allowed under his account will be returned.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct ContractsFor {
    /// Array of available contracts details
    #[serde(rename = "available")]
    pub available: Vec<Availableitem>,
    /// Symbol's next market-close time as an epoch value
    #[serde(rename = "close", skip_serializing_if = "Option::is_none")]
    pub close: Option<Value>,
    /// Indicates the feed license for symbol, for example whether its realtime or delayed
    #[serde(rename = "feed_license", skip_serializing_if = "Option::is_none")]
    pub feed_license: String,
    /// Count of contracts available
    #[serde(rename = "hit_count", skip_serializing_if = "Option::is_none")]
    pub hit_count: f64,
    /// Array of non_available contracts details
    #[serde(rename = "non_available", skip_serializing_if = "Option::is_none")]
    pub non_available: Vec<Value>,
    /// Symbol's next market-open time as an epoch value
    #[serde(rename = "open", skip_serializing_if = "Option::is_none")]
    pub open: Option<Value>,
    /// Current spot price for this underlying
    #[serde(rename = "spot", skip_serializing_if = "Option::is_none")]
    pub spot: Option<Value>,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Availableitem {
    /// Array of available barriers for a predefined trading period
    #[serde(rename = "available_barriers", skip_serializing_if = "Option::is_none")]
    pub available_barriers: Vec<Value>,
    /// Barrier Details.
    #[serde(rename = "barrier", skip_serializing_if = "Option::is_none")]
    pub barrier: Option<Value>,
    /// Category of barrier.
    #[serde(rename = "barrier_category")]
    pub barrier_category: String,
    /// [Only for Vanilla] Barrier Choices
    #[serde(rename = "barrier_choices", skip_serializing_if = "Option::is_none")]
    pub barrier_choices: Vec<Value>,
    /// Number of barriers.
    #[serde(rename = "barriers")]
    pub barriers: f64,
    /// Cancellation range
    #[serde(rename = "cancellation_range", skip_serializing_if = "Option::is_none")]
    pub cancellation_range: Vec<Value>,
    /// Category of contract.
    #[serde(rename = "contract_category")]
    pub contract_category: String,
    /// Category of the contract.
    #[serde(rename = "contract_category_display")]
    pub contract_category_display: String,
    /// Display name for the type of contract.
    #[serde(rename = "contract_display", skip_serializing_if = "Option::is_none")]
    pub contract_display: String,
    /// Type of contract.
    #[serde(rename = "contract_type")]
    pub contract_type: String,
    /// Default stake for the contract
    #[serde(rename = "default_stake", skip_serializing_if = "Option::is_none")]
    pub default_stake: f64,
    /// [Only for Turbos] Its selected payout per point
    #[serde(rename = "display_number_of_contracts", skip_serializing_if = "Option::is_none")]
    pub display_number_of_contracts: f64,
    /// Name of exchange
    #[serde(rename = "exchange_name")]
    pub exchange_name: String,
    /// Array of barriers already expired
    #[serde(rename = "expired_barriers", skip_serializing_if = "Option::is_none")]
    pub expired_barriers: Vec<Value>,
    /// Expiry Type.
    #[serde(rename = "expiry_type")]
    pub expiry_type: String,
    /// Array of returned forward starting options
    #[serde(rename = "forward_starting_options", skip_serializing_if = "Option::is_none")]
    pub forward_starting_options: Vec<ForwardStartingOptionsitem>,
    /// Growth rate range.
    #[serde(rename = "growth_rate_range", skip_serializing_if = "Option::is_none")]
    pub growth_rate_range: Vec<Value>,
    /// High barrier Details.
    #[serde(rename = "high_barrier", skip_serializing_if = "Option::is_none")]
    pub high_barrier: Option<Value>,
    /// Last digit range
    #[serde(rename = "last_digit_range", skip_serializing_if = "Option::is_none")]
    pub last_digit_range: Vec<Value>,
    /// Low barrier Details.
    #[serde(rename = "low_barrier", skip_serializing_if = "Option::is_none")]
    pub low_barrier: Option<Value>,
    /// Type of market.
    #[serde(rename = "market")]
    pub market: String,
    /// Maximum contract duration
    #[serde(rename = "max_contract_duration")]
    pub max_contract_duration: String,
    /// [Only for turbos options] Maximum contract stake
    #[serde(rename = "max_stake", skip_serializing_if = "Option::is_none")]
    pub max_stake: Option<Value>,
    /// Minimum contract duration.
    #[serde(rename = "min_contract_duration")]
    pub min_contract_duration: String,
    /// [Only for turbos options] Minimum contract stake
    #[serde(rename = "min_stake", skip_serializing_if = "Option::is_none")]
    pub min_stake: Option<Value>,
    /// Multiplier range.
    #[serde(rename = "multiplier_range", skip_serializing_if = "Option::is_none")]
    pub multiplier_range: Vec<Value>,
    /// [Only for Turbos] Payout Choices
    #[serde(rename = "payout_choices", skip_serializing_if = "Option::is_none")]
    pub payout_choices: Vec<Value>,
    /// Maximum payout.
    #[serde(rename = "payout_limit", skip_serializing_if = "Option::is_none")]
    pub payout_limit: f64,
    /// Type of sentiment.
    #[serde(rename = "sentiment")]
    pub sentiment: String,
    /// Start Type.
    #[serde(rename = "start_type")]
    pub start_type: String,
    /// Type of submarket.
    #[serde(rename = "submarket")]
    pub submarket: String,
    /// A hash of predefined trading period
    #[serde(rename = "trading_period", skip_serializing_if = "Option::is_none")]
    pub trading_period: TradingPeriod,
    /// Symbol code
    #[serde(rename = "underlying_symbol")]
    pub underlying_symbol: String,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct ForwardStartingOptionsitem {
    /// The epoch value for the blackouts of forward starting session.
    #[serde(rename = "blackouts", skip_serializing_if = "Option::is_none")]
    pub blackouts: Vec<Value>,
    /// The epoch value for the closing date of forward starting session.
    #[serde(rename = "close", skip_serializing_if = "Option::is_none")]
    pub close: String,
    /// The epoch value for the date of forward starting session.
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: String,
    /// The epoch value for the opening date of forward starting session.
    #[serde(rename = "open", skip_serializing_if = "Option::is_none")]
    pub open: String,
}










