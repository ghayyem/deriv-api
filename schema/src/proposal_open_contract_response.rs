
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/proposal_open_contract/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Latest price and other details for an open contract in the user's portfolio
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct ProposalOpenContractResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type", skip_serializing_if = "Option::is_none")]
    pub msg_type: MsgTypeEnum,
    /// Latest price and other details for an open contract
    #[serde(rename = "proposal_open_contract", skip_serializing_if = "Option::is_none")]
    pub proposal_open_contract: ProposalOpenContract,
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
use chrono::{DateTime, Utc};

/// Latest price and other details for an open contract
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct ProposalOpenContract {
    /// Account Id
    #[serde(rename = "account_id", skip_serializing_if = "Option::is_none")]
    pub account_id: f64,
    /// The markup amount charged on a client's stake amount
    #[serde(rename = "app_markup_amount", skip_serializing_if = "Option::is_none")]
    pub app_markup_amount: String,
    /// Tick details around contract start and end time.
    #[serde(rename = "audit_details", skip_serializing_if = "Option::is_none")]
    pub audit_details: Option<Value>,
    /// Barrier of the contract (if any).
    #[serde(rename = "barrier", skip_serializing_if = "Option::is_none")]
    pub barrier: Option<Value>,
    /// The number of barriers a contract has.
    #[serde(rename = "barrier_count", skip_serializing_if = "Option::is_none")]
    pub barrier_count: f64,
    /// [Only for accumulator] Absolute difference between high/low barrier and spot
    #[serde(rename = "barrier_spot_distance", skip_serializing_if = "Option::is_none")]
    pub barrier_spot_distance: String,
    /// Price at which the contract could be sold back to the company.
    #[serde(rename = "bid_price", skip_serializing_if = "Option::is_none")]
    pub bid_price: f64,
    /// Price at which contract was purchased
    #[serde(rename = "buy_price", skip_serializing_if = "Option::is_none")]
    pub buy_price: f64,
    /// Contains information about contract cancellation option.
    #[serde(rename = "cancellation", skip_serializing_if = "Option::is_none")]
    pub cancellation: Cancellation,
    /// Commission in payout currency amount.
    #[serde(rename = "commision", skip_serializing_if = "Option::is_none")]
    pub commision: Option<Value>,
    /// Commission in payout currency amount.
    #[serde(rename = "commission", skip_serializing_if = "Option::is_none")]
    pub commission: Option<Value>,
    /// The internal contract identifier
    #[serde(rename = "contract_id", skip_serializing_if = "Option::is_none")]
    pub contract_id: i64,
    /// Contract type.
    #[serde(rename = "contract_type", skip_serializing_if = "Option::is_none")]
    pub contract_type: String,
    /// The currency code of the contract.
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: String,
    /// Spot value if we have license to stream this symbol.
    #[serde(rename = "current_spot", skip_serializing_if = "Option::is_none")]
    pub current_spot: f64,
    /// Spot value with the correct precision if we have license to stream this symbol.
    #[serde(rename = "current_spot_display_value", skip_serializing_if = "Option::is_none")]
    pub current_spot_display_value: String,
    /// [Applicable for accumulator] High barrier based on current spot.
    #[serde(rename = "current_spot_high_barrier", skip_serializing_if = "Option::is_none")]
    pub current_spot_high_barrier: String,
    /// [Applicable for accumulator] Low barrier based on current spot.
    #[serde(rename = "current_spot_low_barrier", skip_serializing_if = "Option::is_none")]
    pub current_spot_low_barrier: String,
    /// The corresponding time of the current spot.
    #[serde(rename = "current_spot_time", skip_serializing_if = "Option::is_none")]
    pub current_spot_time: i64,
    /// Expiry date (epoch) of the Contract. Please note that it is not applicable for tick trade contracts.
    #[serde(rename = "date_expiry", skip_serializing_if = "Option::is_none")]
    pub date_expiry: i64,
    /// Settlement date (epoch) of the contract.
    #[serde(rename = "date_settlement", skip_serializing_if = "Option::is_none")]
    pub date_settlement: i64,
    /// Start date (epoch) of the contract.
    #[serde(rename = "date_start", skip_serializing_if = "Option::is_none")]
    pub date_start: i64,
    /// Display name of underlying
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: String,
    /// [Only for vanilla or turbos options] The implied number of contracts
    #[serde(rename = "display_number_of_contracts", skip_serializing_if = "Option::is_none")]
    pub display_number_of_contracts: String,
    /// The `bid_price` with the correct precision
    #[serde(rename = "display_value", skip_serializing_if = "Option::is_none")]
    pub display_value: String,
    /// Same as `entry_tick`. For backwards compatibility.
    #[serde(rename = "entry_spot", skip_serializing_if = "Option::is_none")]
    pub entry_spot: Option<Value>,
    /// Same as `entry_tick_display_value`. For backwards compatibility.
    #[serde(rename = "entry_spot_display_value", skip_serializing_if = "Option::is_none")]
    pub entry_spot_display_value: Option<Value>,
    /// This is the entry spot of the contract. For contracts starting immediately it is the next tick after the start time. For forward-starting contracts it is the spot at the start time.
    #[serde(rename = "entry_tick", skip_serializing_if = "Option::is_none")]
    pub entry_tick: f64,
    /// This is the entry spot with the correct precision of the contract. For contracts starting immediately it is the next tick after the start time. For forward-starting contracts it is the spot at the start time.
    #[serde(rename = "entry_tick_display_value", skip_serializing_if = "Option::is_none")]
    pub entry_tick_display_value: String,
    /// This is the epoch time of the entry tick.
    #[serde(rename = "entry_tick_time", skip_serializing_if = "Option::is_none")]
    pub entry_tick_time: i64,
    /// Exit tick can refer to the latest tick at the end time, the tick that fulfils the contract's winning or losing condition for path dependent contracts (Touch/No Touch and Stays Between/Goes Outside) or the tick at which the contract is sold before expiry.
    #[serde(rename = "exit_tick", skip_serializing_if = "Option::is_none")]
    pub exit_tick: f64,
    /// Exit tick can refer to the latest tick at the end time, the tick that fulfils the contract's winning or losing condition for path dependent contracts (Touch/No Touch and Stays Between/Goes Outside) or the tick at which the contract is sold before expiry.
    #[serde(rename = "exit_tick_display_value", skip_serializing_if = "Option::is_none")]
    pub exit_tick_display_value: String,
    /// This is the epoch time of the exit tick. Note that since certain instruments don't tick every second, the exit tick time may be a few seconds before the end time.
    #[serde(rename = "exit_tick_time", skip_serializing_if = "Option::is_none")]
    pub exit_tick_time: i64,
    /// This is the expiry time.
    #[serde(rename = "expiry_time", skip_serializing_if = "Option::is_none")]
    pub expiry_time: i64,
    /// [Only for accumulator] Growth rate of an accumulator contract.
    #[serde(rename = "growth_rate", skip_serializing_if = "Option::is_none")]
    pub growth_rate: f64,
    /// High barrier of the contract (if any).
    #[serde(rename = "high_barrier", skip_serializing_if = "Option::is_none")]
    pub high_barrier: String,
    /// A per-connection unique identifier. Can be passed to the `forget` API call to unsubscribe.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    /// Whether the contract is expired or not.
    #[serde(rename = "is_expired", skip_serializing_if = "Option::is_none")]
    pub is_expired: IsExpiredEnum,
    /// Whether the contract is forward-starting or not.
    #[serde(rename = "is_forward_starting", skip_serializing_if = "Option::is_none")]
    pub is_forward_starting: IsForwardStartingEnum,
    /// Whether the contract is an intraday contract.
    #[serde(rename = "is_intraday", skip_serializing_if = "Option::is_none")]
    pub is_intraday: IsIntradayEnum,
    /// Whether the contract expiry price will depend on the path of the market (e.g. One Touch contract).
    #[serde(rename = "is_path_dependent", skip_serializing_if = "Option::is_none")]
    pub is_path_dependent: IsPathDependentEnum,
    /// Whether the contract is settleable or not.
    #[serde(rename = "is_settleable", skip_serializing_if = "Option::is_none")]
    pub is_settleable: IsSettleableEnum,
    /// Whether the contract is sold or not.
    #[serde(rename = "is_sold", skip_serializing_if = "Option::is_none")]
    pub is_sold: IsSoldEnum,
    /// Whether the contract can be cancelled.
    #[serde(rename = "is_valid_to_cancel", skip_serializing_if = "Option::is_none")]
    pub is_valid_to_cancel: IsValidToCancelEnum,
    /// Whether the contract can be sold back to the company.
    #[serde(rename = "is_valid_to_sell", skip_serializing_if = "Option::is_none")]
    pub is_valid_to_sell: IsValidToSellEnum,
    /// [Optional] Indicator whether take profit, stop loss, and/or stop out is allowed to be updated.
    #[serde(rename = "is_valid_to_update", skip_serializing_if = "Option::is_none")]
    pub is_valid_to_update: Option<Value>,
    /// Orders are applicable to `MULTUP` and `MULTDOWN` contracts only.
    #[serde(rename = "limit_order", skip_serializing_if = "Option::is_none")]
    pub limit_order: LimitOrder,
    /// Text description of the contract purchased, Example: Win payout if Volatility 100 Index is strictly higher than entry spot at 10 minutes after contract start time.
    #[serde(rename = "longcode", skip_serializing_if = "Option::is_none")]
    pub longcode: String,
    /// Low barrier of the contract (if any).
    #[serde(rename = "low_barrier", skip_serializing_if = "Option::is_none")]
    pub low_barrier: String,
    /// [Only for lookback trades] Multiplier applies when calculating the final payoff for each type of lookback. e.g. (Exit spot - Lowest historical price) * multiplier = Payout
    #[serde(rename = "multiplier", skip_serializing_if = "Option::is_none")]
    pub multiplier: f64,
    /// Payout value of the contract.
    #[serde(rename = "payout", skip_serializing_if = "Option::is_none")]
    pub payout: f64,
    /// The latest bid price minus buy price.
    #[serde(rename = "profit", skip_serializing_if = "Option::is_none")]
    pub profit: f64,
    /// Profit in percentage.
    #[serde(rename = "profit_percentage", skip_serializing_if = "Option::is_none")]
    pub profit_percentage: f64,
    /// Epoch of purchase time, will be same as `date_start` for all contracts except forward starting contracts.
    #[serde(rename = "purchase_time", skip_serializing_if = "Option::is_none")]
    pub purchase_time: i64,
    /// [Only for reset trades i.e. RESETCALL and RESETPUT] Reset barrier of the contract.
    #[serde(rename = "reset_barrier", skip_serializing_if = "Option::is_none")]
    pub reset_barrier: Option<Value>,
    /// [Only for reset trades i.e. RESETCALL and RESETPUT] The epoch time of a barrier reset.
    #[serde(rename = "reset_time", skip_serializing_if = "Option::is_none")]
    pub reset_time: i64,
    /// Spot value at the selected tick for the contract.
    #[serde(rename = "selected_spot", skip_serializing_if = "Option::is_none")]
    pub selected_spot: f64,
    /// [Only for highlowticks trades i.e. TICKHIGH and TICKLOW] Selected tick for the contract.
    #[serde(rename = "selected_tick", skip_serializing_if = "Option::is_none")]
    pub selected_tick: i64,
    /// Price at which contract was sold, only available when contract has been sold.
    #[serde(rename = "sell_price", skip_serializing_if = "Option::is_none")]
    pub sell_price: f64,
    /// Latest spot value at the sell time. (only present for contracts already sold). Will no longer be supported in the next API release.
    #[serde(rename = "sell_spot", skip_serializing_if = "Option::is_none")]
    pub sell_spot: f64,
    /// Latest spot value with the correct precision at the sell time. (only present for contracts already sold). Will no longer be supported in the next API release.
    #[serde(rename = "sell_spot_display_value", skip_serializing_if = "Option::is_none")]
    pub sell_spot_display_value: String,
    /// Epoch time of the sell spot. Note that since certain underlyings don't tick every second, the sell spot time may be a few seconds before the sell time. (only present for contracts already sold). Will no longer be supported in the next API release.
    #[serde(rename = "sell_spot_time", skip_serializing_if = "Option::is_none")]
    pub sell_spot_time: i64,
    /// Epoch time of when the contract was sold (only present for contracts already sold)
    #[serde(rename = "sell_time", skip_serializing_if = "Option::is_none")]
    pub sell_time: Option<Value>,
    /// Coded description of the contract purchased.
    #[serde(rename = "shortcode", skip_serializing_if = "Option::is_none")]
    pub shortcode: String,
    /// Contract status. Will be `sold` if the contract was sold back before expiry, `won` if won and `lost` if lost at expiry. Otherwise will be `open`
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<StatusEnum>,
    /// Only for tick trades, number of ticks
    #[serde(rename = "tick_count", skip_serializing_if = "Option::is_none")]
    pub tick_count: i64,
    /// [Only for accumulator] Number of ticks passed since entry_tick
    #[serde(rename = "tick_passed", skip_serializing_if = "Option::is_none")]
    pub tick_passed: i64,
    /// Tick stream from entry to end time.
    #[serde(rename = "tick_stream", skip_serializing_if = "Option::is_none")]
    pub tick_stream: Vec<TickStreamitem>,
    /// Every contract has buy and sell transaction ids, i.e. when you purchase a contract we associate it with buy transaction id, and if contract is already sold we associate that with sell transaction id.
    #[serde(rename = "transaction_ids", skip_serializing_if = "Option::is_none")]
    pub transaction_ids: TransactionIds,
    /// The underlying symbol code.
    #[serde(rename = "underlying", skip_serializing_if = "Option::is_none")]
    pub underlying: String,
    /// Error message if validation fails
    #[serde(rename = "validation_error", skip_serializing_if = "Option::is_none")]
    pub validation_error: String,
    /// Error code if validation fails
    #[serde(rename = "validation_error_code", skip_serializing_if = "Option::is_none")]
    pub validation_error_code: String,
    /// Contains contract validation information.
    #[serde(rename = "validation_params", skip_serializing_if = "Option::is_none")]
    pub validation_params: ValidationParams,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct TickStreamitem {
    /// Epoch time of a tick or the contract start or end time.
    #[serde(rename = "epoch", skip_serializing_if = "Option::is_none")]
    pub epoch: i64,
    /// The spot value at the given epoch.
    #[serde(rename = "tick", skip_serializing_if = "Option::is_none")]
    pub tick: Option<Value>,
    /// The spot value with the correct precision at the given epoch.
    #[serde(rename = "tick_display_value", skip_serializing_if = "Option::is_none")]
    pub tick_display_value: Option<Value>,
}






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Every contract has buy and sell transaction ids, i.e. when you purchase a contract we associate it with buy transaction id, and if contract is already sold we associate that with sell transaction id.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct TransactionIds {
    /// Buy transaction ID for that contract
    #[serde(rename = "buy", skip_serializing_if = "Option::is_none")]
    pub buy: i64,
    /// Sell transaction ID for that contract, only present when contract is already sold.
    #[serde(rename = "sell", skip_serializing_if = "Option::is_none")]
    pub sell: i64,
}






/// Whether the contract is expired or not.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IsExpiredEnum {
    Value0,
    Value1 = 1,
}

impl IsExpiredEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Value0 => "0",
            Self::Value1 => "1",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "0" => Some(Self::Value0),
            "1" => Some(Self::Value1),
            _ => None,
        }
    }
}
/// Whether the contract is forward-starting or not.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IsForwardStartingEnum {
    Value0,
    Value1 = 1,
}

impl IsForwardStartingEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Value0 => "0",
            Self::Value1 => "1",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "0" => Some(Self::Value0),
            "1" => Some(Self::Value1),
            _ => None,
        }
    }
}
/// Whether the contract is an intraday contract.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IsIntradayEnum {
    Value0,
    Value1 = 1,
}

impl IsIntradayEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Value0 => "0",
            Self::Value1 => "1",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "0" => Some(Self::Value0),
            "1" => Some(Self::Value1),
            _ => None,
        }
    }
}
/// Whether the contract expiry price will depend on the path of the market (e.g. One Touch contract).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IsPathDependentEnum {
    Value0,
    Value1 = 1,
}

impl IsPathDependentEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Value0 => "0",
            Self::Value1 => "1",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "0" => Some(Self::Value0),
            "1" => Some(Self::Value1),
            _ => None,
        }
    }
}
/// Whether the contract is settleable or not.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IsSettleableEnum {
    Value0,
    Value1 = 1,
}

impl IsSettleableEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Value0 => "0",
            Self::Value1 => "1",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "0" => Some(Self::Value0),
            "1" => Some(Self::Value1),
            _ => None,
        }
    }
}
/// Whether the contract is sold or not.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IsSoldEnum {
    Value0,
    Value1 = 1,
}

impl IsSoldEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Value0 => "0",
            Self::Value1 => "1",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "0" => Some(Self::Value0),
            "1" => Some(Self::Value1),
            _ => None,
        }
    }
}
/// Whether the contract can be cancelled.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IsValidToCancelEnum {
    Value0,
    Value1 = 1,
}

impl IsValidToCancelEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Value0 => "0",
            Self::Value1 => "1",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "0" => Some(Self::Value0),
            "1" => Some(Self::Value1),
            _ => None,
        }
    }
}
/// Whether the contract can be sold back to the company.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IsValidToSellEnum {
    Value0,
    Value1 = 1,
}

impl IsValidToSellEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Value0 => "0",
            Self::Value1 => "1",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "0" => Some(Self::Value0),
            "1" => Some(Self::Value1),
            _ => None,
        }
    }
}


