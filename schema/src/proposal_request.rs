
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/proposal/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Gets latest price for a specific contract.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct ProposalRequest {
    /// [Optional] Proposed contract payout or stake, or multiplier (for lookbacks).
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: f64,
    /// [Optional] Barrier for the contract (or last digit prediction for digit contracts). Contracts less than 24 hours in duration would need a relative barrier (barriers which need +/-), where entry spot would be adjusted accordingly with that amount to define a barrier, except for Synthetic Indices as they support both relative and absolute barriers. Not needed for lookbacks.
    #[serde(rename = "barrier", skip_serializing_if = "Option::is_none")]
    pub barrier: String,
    /// [Optional] Low barrier for the contract (for contracts with two barriers). Contracts less than 24 hours in duration would need a relative barrier (barriers which need +/-), where entry spot would be adjusted accordingly with that amount to define a barrier, except for Synthetic Indices as they support both relative and absolute barriers. Not needed for lookbacks.
    #[serde(rename = "barrier2", skip_serializing_if = "Option::is_none")]
    pub barrier_2: String,
    /// [Optional] Barrier range for callputspread.
    #[serde(rename = "barrier_range", skip_serializing_if = "Option::is_none")]
    pub barrier_range: BarrierRangeEnum,
    /// [Optional] Indicates type of the `amount`.
    #[serde(rename = "basis", skip_serializing_if = "Option::is_none")]
    pub basis: BasisEnum,
    /// Cancellation duration option (only for `MULTUP` and `MULTDOWN` contracts).
    #[serde(rename = "cancellation", skip_serializing_if = "Option::is_none")]
    pub cancellation: String,
    /// The proposed contract type
    #[serde(rename = "contract_type")]
    pub contract_type: ContractTypeEnum,
    /// This can only be the account-holder's currency (obtained from `payout_currencies` call).
    #[serde(rename = "currency")]
    pub currency: String,
    /// [Optional] Epoch value of the expiry time of the contract. Either date_expiry or duration is required.
    #[serde(rename = "date_expiry", skip_serializing_if = "Option::is_none")]
    pub date_expiry: i64,
    /// [Optional] Indicates epoch value of the starting time of the contract. If left empty, the start time of the contract is now.
    #[serde(rename = "date_start", skip_serializing_if = "Option::is_none")]
    pub date_start: i64,
    /// [Optional] Duration quantity. Either date_expiry or duration is required.
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: i64,
    /// [Optional] Duration unit - `s`: seconds, `m`: minutes, `h`: hours, `d`: days, `t`: ticks.
    #[serde(rename = "duration_unit", skip_serializing_if = "Option::is_none")]
    pub duration_unit: DurationUnitEnum,
    /// [Optional] Growth rate of an accumulator contract.
    #[serde(rename = "growth_rate", skip_serializing_if = "Option::is_none")]
    pub growth_rate: f64,
    /// Add an order to close the contract once the order condition is met (only for `MULTUP` and `MULTDOWN` and 'ACCU' contracts). Supported orders: `take_profit`, `stop_loss`.
    #[serde(rename = "limit_order", skip_serializing_if = "Option::is_none")]
    pub limit_order: LimitOrder,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// [Optional] The multiplier for non-binary options. E.g. lookbacks.
    #[serde(rename = "multiplier", skip_serializing_if = "Option::is_none")]
    pub multiplier: f64,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] Clients can provide payout_per_point directly, and the barrier will be calculated based on this payout_per_point value.
    #[serde(rename = "payout_per_point", skip_serializing_if = "Option::is_none")]
    pub payout_per_point: f64,
    /// [Optional] The product type.
    #[serde(rename = "product_type", skip_serializing_if = "Option::is_none")]
    pub product_type: ProductTypeEnum,
    /// Must be `1`
    #[serde(rename = "proposal")]
    pub proposal: ProposalEnum,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// [Optional] The tick that is predicted to have the highest/lowest value - for `TICKHIGH` and `TICKLOW` contracts.
    #[serde(rename = "selected_tick", skip_serializing_if = "Option::is_none")]
    pub selected_tick: i64,
    /// [Optional] 1 - to initiate a realtime stream of prices. Note that tick trades (without a user-defined barrier), digit trades and less than 24 hours at-the-money contracts for the following underlying symbols are not streamed: `R_10`, `R_25`, `R_50`, `R_75`, `R_100`, `RDBULL`, `RDBEAR` (this is because their price is constant).
    #[serde(rename = "subscribe", skip_serializing_if = "Option::is_none")]
    pub subscribe: SubscribeEnum,
    /// The short symbol name (obtained from `active_symbols` call).
    #[serde(rename = "symbol")]
    pub symbol: String,
    /// [Optional] Required only for multi-barrier trading. Defines the epoch value of the trading period start time.
    #[serde(rename = "trading_period_start", skip_serializing_if = "Option::is_none")]
    pub trading_period_start: i64,
}




/// [Optional] Barrier range for callputspread.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BarrierRangeEnum {
    Tight,
    Middle,
    Wide,
}

impl BarrierRangeEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Tight => "tight",
            Self::Middle => "middle",
            Self::Wide => "wide",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "tight" => Some(Self::Tight),
            "middle" => Some(Self::Middle),
            "wide" => Some(Self::Wide),
            _ => None,
        }
    }
}
/// [Optional] The product type.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProductTypeEnum {
    Basic,
}

impl ProductTypeEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Basic => "basic",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "basic" => Some(Self::Basic),
            _ => None,
        }
    }
}
/// Must be `1`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProposalEnum {
    Value1 = 1,
}

impl ProposalEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Value1 => "1",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "1" => Some(Self::Value1),
            _ => None,
        }
    }
}
