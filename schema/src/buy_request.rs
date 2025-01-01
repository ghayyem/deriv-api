
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/buy/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Buy a Contract
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct BuyRequest {
    /// Either the ID received from a Price Proposal (`proposal` call), or `1` if contract buy parameters are passed in the `parameters` field.
    #[serde(rename = "buy")]
    pub buy: String,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// [Optional] Used to pass the parameters for contract buy.
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Parameters,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// Maximum price at which to purchase the contract.
    #[serde(rename = "price")]
    pub price: f64,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// [Optional] `1` to stream.
    #[serde(rename = "subscribe", skip_serializing_if = "Option::is_none")]
    pub subscribe: SubscribeEnum,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// [Optional] Used to pass the parameters for contract buy.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Parameters {
    /// [Optional] Proposed payout or stake value
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: f64,
    /// [Optional] Markup added to contract prices (as a percentage of contract payout)
    #[serde(rename = "app_markup_percentage", skip_serializing_if = "Option::is_none")]
    pub app_markup_percentage: f64,
    /// [Optional] Barrier for the contract (or last digit prediction for digit contracts). Contracts less than 24 hours in duration would need a relative barrier (barriers which need +/-), where entry spot would be adjusted accordingly with that amount to define a barrier, except for Synthetic Indices as they support both relative and absolute barriers.
    #[serde(rename = "barrier", skip_serializing_if = "Option::is_none")]
    pub barrier: String,
    /// [Optional] Low barrier for the contract (for contracts with two barriers). Contracts less than 24 hours in duration would need a relative barrier (barriers which need +/-), where entry spot would be adjusted accordingly with that amount to define a barrier, except for Synthetic Indices as they support both relative and absolute barriers.
    #[serde(rename = "barrier2", skip_serializing_if = "Option::is_none")]
    pub barrier_2: String,
    /// [Optional] Barrier range for callputspread.
    #[serde(rename = "barrier_range", skip_serializing_if = "Option::is_none")]
    pub barrier_range: BarrierRangeEnum,
    /// [Optional] Indicates whether amount is 'payout' or 'stake' for binary options.
    #[serde(rename = "basis", skip_serializing_if = "Option::is_none")]
    pub basis: BasisEnum,
    /// Cancellation duration option (only for `MULTUP` and `MULTDOWN` contracts).
    #[serde(rename = "cancellation", skip_serializing_if = "Option::is_none")]
    pub cancellation: String,
    /// A valid contract-type
    #[serde(rename = "contract_type")]
    pub contract_type: ContractTypeEnum,
    /// This can only be the account-holder's currency
    #[serde(rename = "currency")]
    pub currency: String,
    /// [Optional] Epoch value of the expiry time of the contract. You must either specify date_expiry or duration.
    #[serde(rename = "date_expiry", skip_serializing_if = "Option::is_none")]
    pub date_expiry: i64,
    /// [Optional] For forward-starting contracts, epoch value of the starting time of the contract.
    #[serde(rename = "date_start", skip_serializing_if = "Option::is_none")]
    pub date_start: i64,
    /// [Optional] Duration quantity
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: i64,
    /// [Optional] Duration unit is `s`: seconds, `m`: minutes, `h`: hours, `d`: days, `t`: ticks
    #[serde(rename = "duration_unit", skip_serializing_if = "Option::is_none")]
    pub duration_unit: DurationUnitEnum,
    /// [Optional] Growth rate of an accumulator contract.
    #[serde(rename = "growth_rate", skip_serializing_if = "Option::is_none")]
    pub growth_rate: f64,
    /// Add an order to close the contract once the order condition is met (only for `MULTUP` and `MULTDOWN` and `ACCU` contracts).
    #[serde(rename = "limit_order", skip_serializing_if = "Option::is_none")]
    pub limit_order: LimitOrder,
    /// [Optional] The multiplier for non-binary options. E.g. lookbacks.
    #[serde(rename = "multiplier", skip_serializing_if = "Option::is_none")]
    pub multiplier: f64,
    /// [Optional] Clients can provide payout_per_point directly, and the barrier will be calculated based on this payout_per_point value.
    #[serde(rename = "payout_per_point", skip_serializing_if = "Option::is_none")]
    pub payout_per_point: f64,
    /// [Optional] The product type.
    #[serde(rename = "product_type", skip_serializing_if = "Option::is_none")]
    pub product_type: ProductTypeEnum,
    /// [Optional] The tick that is predicted to have the highest/lowest value - for tickhigh and ticklow contracts.
    #[serde(rename = "selected_tick", skip_serializing_if = "Option::is_none")]
    pub selected_tick: i64,
    /// Symbol code
    #[serde(rename = "symbol")]
    pub symbol: String,
    /// [Optional] An epoch value of a predefined trading period start time
    #[serde(rename = "trading_period_start", skip_serializing_if = "Option::is_none")]
    pub trading_period_start: i64,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Add an order to close the contract once the order condition is met (only for `MULTUP` and `MULTDOWN` and `ACCU` contracts).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct LimitOrder {
    /// Contract will be automatically closed when the value of the contract reaches a specific loss.
    #[serde(rename = "stop_loss", skip_serializing_if = "Option::is_none")]
    pub stop_loss: f64,
    /// Contract will be automatically closed when the value of the contract reaches a specific profit.
    #[serde(rename = "take_profit", skip_serializing_if = "Option::is_none")]
    pub take_profit: f64,
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
/// [Optional] Indicates whether amount is 'payout' or 'stake' for binary options.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BasisEnum {
    Payout,
    Stake,
}

impl BasisEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Payout => "payout",
            Self::Stake => "stake",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "payout" => Some(Self::Payout),
            "stake" => Some(Self::Stake),
            _ => None,
        }
    }
}
/// A valid contract-type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContractTypeEnum {
    MULTUP,
    MULTDOWN,
    UPORDOWN,
    EXPIRYRANGE,
    ONETOUCH,
    CALLE,
    LBHIGHLOW,
    ASIAND,
    EXPIRYRANGEE,
    DIGITDIFF,
    DIGITMATCH,
    DIGITOVER,
    PUTE,
    DIGITUNDER,
    NOTOUCH,
    CALL,
    RANGE,
    LBFLOATPUT,
    DIGITODD,
    PUT,
    ASIANU,
    LBFLOATCALL,
    EXPIRYMISSE,
    EXPIRYMISS,
    DIGITEVEN,
    TICKHIGH,
    TICKLOW,
    RESETCALL,
    RESETPUT,
    CALLSPREAD,
    PUTSPREAD,
    RUNHIGH,
    RUNLOW,
    ACCU,
    VANILLALONGCALL,
    VANILLALONGPUT,
    TURBOSLONG,
    TURBOSSHORT,
}

impl ContractTypeEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MULTUP => "MULTUP",
            Self::MULTDOWN => "MULTDOWN",
            Self::UPORDOWN => "UPORDOWN",
            Self::EXPIRYRANGE => "EXPIRYRANGE",
            Self::ONETOUCH => "ONETOUCH",
            Self::CALLE => "CALLE",
            Self::LBHIGHLOW => "LBHIGHLOW",
            Self::ASIAND => "ASIAND",
            Self::EXPIRYRANGEE => "EXPIRYRANGEE",
            Self::DIGITDIFF => "DIGITDIFF",
            Self::DIGITMATCH => "DIGITMATCH",
            Self::DIGITOVER => "DIGITOVER",
            Self::PUTE => "PUTE",
            Self::DIGITUNDER => "DIGITUNDER",
            Self::NOTOUCH => "NOTOUCH",
            Self::CALL => "CALL",
            Self::RANGE => "RANGE",
            Self::LBFLOATPUT => "LBFLOATPUT",
            Self::DIGITODD => "DIGITODD",
            Self::PUT => "PUT",
            Self::ASIANU => "ASIANU",
            Self::LBFLOATCALL => "LBFLOATCALL",
            Self::EXPIRYMISSE => "EXPIRYMISSE",
            Self::EXPIRYMISS => "EXPIRYMISS",
            Self::DIGITEVEN => "DIGITEVEN",
            Self::TICKHIGH => "TICKHIGH",
            Self::TICKLOW => "TICKLOW",
            Self::RESETCALL => "RESETCALL",
            Self::RESETPUT => "RESETPUT",
            Self::CALLSPREAD => "CALLSPREAD",
            Self::PUTSPREAD => "PUTSPREAD",
            Self::RUNHIGH => "RUNHIGH",
            Self::RUNLOW => "RUNLOW",
            Self::ACCU => "ACCU",
            Self::VANILLALONGCALL => "VANILLALONGCALL",
            Self::VANILLALONGPUT => "VANILLALONGPUT",
            Self::TURBOSLONG => "TURBOSLONG",
            Self::TURBOSSHORT => "TURBOSSHORT",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "MULTUP" => Some(Self::MULTUP),
            "MULTDOWN" => Some(Self::MULTDOWN),
            "UPORDOWN" => Some(Self::UPORDOWN),
            "EXPIRYRANGE" => Some(Self::EXPIRYRANGE),
            "ONETOUCH" => Some(Self::ONETOUCH),
            "CALLE" => Some(Self::CALLE),
            "LBHIGHLOW" => Some(Self::LBHIGHLOW),
            "ASIAND" => Some(Self::ASIAND),
            "EXPIRYRANGEE" => Some(Self::EXPIRYRANGEE),
            "DIGITDIFF" => Some(Self::DIGITDIFF),
            "DIGITMATCH" => Some(Self::DIGITMATCH),
            "DIGITOVER" => Some(Self::DIGITOVER),
            "PUTE" => Some(Self::PUTE),
            "DIGITUNDER" => Some(Self::DIGITUNDER),
            "NOTOUCH" => Some(Self::NOTOUCH),
            "CALL" => Some(Self::CALL),
            "RANGE" => Some(Self::RANGE),
            "LBFLOATPUT" => Some(Self::LBFLOATPUT),
            "DIGITODD" => Some(Self::DIGITODD),
            "PUT" => Some(Self::PUT),
            "ASIANU" => Some(Self::ASIANU),
            "LBFLOATCALL" => Some(Self::LBFLOATCALL),
            "EXPIRYMISSE" => Some(Self::EXPIRYMISSE),
            "EXPIRYMISS" => Some(Self::EXPIRYMISS),
            "DIGITEVEN" => Some(Self::DIGITEVEN),
            "TICKHIGH" => Some(Self::TICKHIGH),
            "TICKLOW" => Some(Self::TICKLOW),
            "RESETCALL" => Some(Self::RESETCALL),
            "RESETPUT" => Some(Self::RESETPUT),
            "CALLSPREAD" => Some(Self::CALLSPREAD),
            "PUTSPREAD" => Some(Self::PUTSPREAD),
            "RUNHIGH" => Some(Self::RUNHIGH),
            "RUNLOW" => Some(Self::RUNLOW),
            "ACCU" => Some(Self::ACCU),
            "VANILLALONGCALL" => Some(Self::VANILLALONGCALL),
            "VANILLALONGPUT" => Some(Self::VANILLALONGPUT),
            "TURBOSLONG" => Some(Self::TURBOSLONG),
            "TURBOSSHORT" => Some(Self::TURBOSSHORT),
            _ => None,
        }
    }
}
/// [Optional] Duration unit is `s`: seconds, `m`: minutes, `h`: hours, `d`: days, `t`: ticks
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DurationUnitEnum {
    D,
    M,
    S,
    H,
    T,
}

impl DurationUnitEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::D => "d",
            Self::M => "m",
            Self::S => "s",
            Self::H => "h",
            Self::T => "t",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "d" => Some(Self::D),
            "m" => Some(Self::M),
            "s" => Some(Self::S),
            "h" => Some(Self::H),
            "t" => Some(Self::T),
            _ => None,
        }
    }
}


