
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/proposal_open_contract/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 
use serde_json::Value;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

// Import shared types from the *same* crate
use crate::is_settleable_enum::IsSettleableEnum; 
use crate::tick_stream_item::TickStreamItem; 
use crate::transaction_ids::TransactionIds; 
use crate::is_path_dependent_enum::IsPathDependentEnum; 
use crate::is_valid_to_cancel_enum::IsValidToCancelEnum; 
use crate::is_sold_enum::IsSoldEnum; 
use crate::status_enum::StatusEnum; 
use crate::is_forward_starting_enum::IsForwardStartingEnum; 
use crate::is_valid_to_sell_enum::IsValidToSellEnum; 
use crate::trade_risk_profile_enum::TradeRiskProfileEnum; 
use crate::limit_order::LimitOrder; 
use crate::validation_params::ValidationParams; 
use crate::cancellation::Cancellation; 
use crate::is_expired_enum::IsExpiredEnum; 
use crate::is_intraday_enum::IsIntradayEnum; 

// It's a struct
/// Latest price and other details for an open contract
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProposalOpenContract {
    /// Account Id\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub account_id: Option<f64>,
    /// The markup amount charged on a client's stake amount\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub app_markup_amount: Option<f64>,
    /// Tick details around contract start and end time.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub audit_details: Option<Value>,
    /// Barrier of the contract (if any).\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub barrier: Option<String>,
    /// The number of barriers a contract has.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub barrier_count: Option<f64>,
    /// [Only for accumulator] Absolute difference between high/low barrier and spot\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub barrier_spot_distance: Option<String>,
    /// Price at which the contract could be sold back to the company.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub bid_price: Option<f64>,
    /// Price at which contract was purchased\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub buy_price: Option<f64>,
    /// Contains information about contract cancellation option.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub cancellation: Option<Cancellation>,
    /// The caution price for the Snowball contract. Breaching this price will reset the coupons accrued to 0.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub caution_price: Option<f64>,
    /// Commission in payout currency amount.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub commision: Option<f64>,
    /// Commission in payout currency amount.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub commission: Option<f64>,
    /// The internal contract identifier\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub contract_id: Option<i64>,
    /// Contract type.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub contract_type: Option<String>,
    /// The epoch times at which the coupons will be accrued for the Snowball contract.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub coupon_collection_epochs: Option<Vec<i64>>,
    /// The coupon rate for the Snowball contract at which the stake will grow for each coupon accrued.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub coupon_rate: Option<String>,
    /// The currency code of the contract.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub currency: Option<String>,
    /// Spot value if we have license to stream this symbol.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub current_spot: Option<f64>,
    /// Spot value with the correct precision if we have license to stream this symbol.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub current_spot_display_value: Option<f64>,
    /// [Applicable for accumulator] High barrier based on current spot.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub current_spot_high_barrier: Option<String>,
    /// [Applicable for accumulator] Low barrier based on current spot.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub current_spot_low_barrier: Option<String>,
    /// The corresponding time of the current spot.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub current_spot_time: Option<i64>,
    /// Expiry date (epoch) of the Contract. Please note that it is not applicable for tick trade contracts.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub date_expiry: Option<DateTime<Utc>>,
    /// Settlement date (epoch) of the contract.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub date_settlement: Option<DateTime<Utc>>,
    /// Start date (epoch) of the contract.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub date_start: Option<DateTime<Utc>>,
    /// Display name of underlying\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub display_name: Option<String>,
    /// [Only for vanilla or turbos options] The implied number of contracts\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub display_number_of_contracts: Option<String>,
    /// The `bid_price` with the correct precision\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub display_value: Option<f64>,
    /// Same as `entry_tick`. For backwards compatibility.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub entry_spot: Option<f64>,
    /// Same as `entry_tick_display_value`. For backwards compatibility.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub entry_spot_display_value: Option<f64>,
    /// This is the entry spot of the contract. For contracts starting immediately it is the next tick after the start time. For forward-starting contracts it is the spot at the start time.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub entry_tick: Option<f64>,
    /// This is the entry spot with the correct precision of the contract. For contracts starting immediately it is the next tick after the start time. For forward-starting contracts it is the spot at the start time.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub entry_tick_display_value: Option<String>,
    /// This is the epoch time of the entry tick.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub entry_tick_time: Option<DateTime<Utc>>,
    /// Exit tick can refer to the latest tick at the end time, the tick that fulfils the contract's winning or losing condition for path dependent contracts (Touch/No Touch and Stays Between/Goes Outside) or the tick at which the contract is sold before expiry.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub exit_tick: Option<f64>,
    /// Exit tick can refer to the latest tick at the end time, the tick that fulfils the contract's winning or losing condition for path dependent contracts (Touch/No Touch and Stays Between/Goes Outside) or the tick at which the contract is sold before expiry.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub exit_tick_display_value: Option<String>,
    /// This is the epoch time of the exit tick. Note that since certain instruments don't tick every second, the exit tick time may be a few seconds before the end time.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub exit_tick_time: Option<DateTime<Utc>>,
    /// This is the expiry time.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub expiry_time: Option<i64>,
    /// [Only for accumulator] Growth rate of an accumulator contract.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub growth_rate: Option<f64>,
    /// High barrier of the contract (if any).\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub high_barrier: Option<String>,
    /// A per-connection unique identifier. Can be passed to the `forget` API call to unsubscribe.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub id: Option<String>,
    /// Whether the contract is expired or not.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub is_expired: Option<IsExpiredEnum>,
    /// Whether the contract is forward-starting or not.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub is_forward_starting: Option<IsForwardStartingEnum>,
    /// Whether the contract is an intraday contract.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub is_intraday: Option<IsIntradayEnum>,
    /// Whether the contract expiry price will depend on the path of the market (e.g. One Touch contract).\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub is_path_dependent: Option<IsPathDependentEnum>,
    /// Whether the contract is settleable or not.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub is_settleable: Option<IsSettleableEnum>,
    /// Whether the contract is sold or not.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub is_sold: Option<IsSoldEnum>,
    /// Whether the contract can be cancelled.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub is_valid_to_cancel: Option<IsValidToCancelEnum>,
    /// Whether the contract can be sold back to the company.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub is_valid_to_sell: Option<IsValidToSellEnum>,
    /// [Optional] Indicator whether take profit, stop loss, and/or stop out is allowed to be updated.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub is_valid_to_update: Option<Value>,
    /// Orders are applicable to `MULTUP` and `MULTDOWN` contracts only.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub limit_order: Option<LimitOrder>,
    /// Text description of the contract purchased, Example: Win payout if Volatility 100 Index is strictly higher than entry spot at 10 minutes after contract start time.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub longcode: Option<String>,
    /// Low barrier of the contract (if any).\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub low_barrier: Option<String>,
    /// [Only for lookback trades] Multiplier applies when calculating the final payoff for each type of lookback. e.g. (Exit spot - Lowest historical price) * multiplier = Payout\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub multiplier: Option<f64>,
    /// The maximum number of coupons available for the Snowball contract.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub num_of_coupons: Option<i64>,
    /// Payout value of the contract.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub payout: Option<f64>,
    /// The latest bid price minus buy price.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub profit: Option<f64>,
    /// Profit in percentage.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub profit_percentage: Option<f64>,
    /// The profit price for the Snowball contract. Breaching this price will close the contract immediately.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub profit_price: Option<f64>,
    /// Epoch of purchase time, will be same as `date_start` for all contracts except forward starting contracts.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub purchase_time: Option<i64>,
    /// [Only for reset trades i.e. RESETCALL and RESETPUT] Reset barrier of the contract.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub reset_barrier: Option<String>,
    /// [Only for reset trades i.e. RESETCALL and RESETPUT] The epoch time of a barrier reset.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub reset_time: Option<DateTime<Utc>>,
    /// Spot value at the selected tick for the contract.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub selected_spot: Option<f64>,
    /// [Only for highlowticks trades i.e. TICKHIGH and TICKLOW] Selected tick for the contract.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub selected_tick: Option<i64>,
    /// Price at which contract was sold, only available when contract has been sold.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub sell_price: Option<f64>,
    /// Latest spot value at the sell time. (only present for contracts already sold). Will no longer be supported in the next API release.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub sell_spot: Option<f64>,
    /// Latest spot value with the correct precision at the sell time. (only present for contracts already sold). Will no longer be supported in the next API release.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub sell_spot_display_value: Option<f64>,
    /// Epoch time of the sell spot. Note that since certain underlyings don't tick every second, the sell spot time may be a few seconds before the sell time. (only present for contracts already sold). Will no longer be supported in the next API release.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub sell_spot_time: Option<i64>,
    /// Epoch time of when the contract was sold (only present for contracts already sold)\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub sell_time: Option<i64>,
    /// Coded description of the contract purchased.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub shortcode: Option<String>,
    /// Contract status. Will be `sold` if the contract was sold back before expiry, `won` if won and `lost` if lost at expiry. Otherwise will be `open`\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub status: Option<StatusEnum>,
    /// Only for tick trades, number of ticks\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub tick_count: Option<i64>,
    /// [Only for accumulator] Number of ticks passed since entry_tick\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub tick_passed: Option<i64>,
    /// Tick stream from entry to end time.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub tick_stream: Option<Vec<TickStreamItem>>,
    /// The trade risk profile for the Snowball contract.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub trade_risk_profile: Option<TradeRiskProfileEnum>,
    /// Every contract has buy and sell transaction ids, i.e. when you purchase a contract we associate it with buy transaction id, and if contract is already sold we associate that with sell transaction id.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub transaction_ids: Option<TransactionIds>,
    /// The underlying symbol code.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub underlying: Option<String>,
    /// Error message if validation fails\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub validation_error: Option<String>,
    /// Error code if validation fails\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub validation_error_code: Option<String>,
    /// Contains contract validation information.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub validation_params: Option<ValidationParams>,
}

