
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/buy_contract_for_multiple_accounts/send.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 


use chrono::{DateTime, Utc};

// Import shared types from the *same* crate
use crate::basis_enum::BasisEnum; 
use crate::duration_unit_enum::DurationUnitEnum; 
use crate::contract_type_enum::ContractTypeEnum; 

// It's a struct
/// [Optional] Used to pass the parameters for contract buy.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Parameters {
    /// [Optional] Proposed `payout` or `stake` value\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub amount: Option<f64>,
    /// [Optional] Markup added to contract prices (as a percentage of contract payout)\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub app_markup_percentage: Option<f64>,
    /// [Optional] Barrier for the contract (or last digit prediction for digit contracts). Contracts less than 24 hours in duration would need a relative barrier (barriers which need +/-), where entry spot would be adjusted accordingly with that amount to define a barrier, except for Synthetic Indices as they support both relative and absolute barriers.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub barrier: Option<f64>,
    /// [Optional] Low barrier for the contract (for contracts with two barriers). Contracts less than 24 hours in duration would need a relative barrier (barriers which need +/-), where entry spot would be adjusted accordingly with that amount to define a barrier, except for Synthetic Indices as they support both relative and absolute barriers.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub barrier2: Option<f64>,
    /// [Optional] Indicate whether amount is 'payout' or 'stake'.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub basis: Option<BasisEnum>,
    /// A valid contract-type\n
    // Correct serde attribute construction - Use helper
    
    pub contract_type: ContractTypeEnum,
    /// This can only be the account-holder's currency\n
    // Correct serde attribute construction - Use helper
    
    pub currency: String,
    /// [Optional] Epoch value of the expiry time of the contract. You must either specify `date_expiry` or `duration`.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub date_expiry: Option<i64>,
    /// [Optional] For forward-starting contracts, epoch value of the starting time of the contract.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub date_start: Option<DateTime<Utc>>,
    /// [Optional] Duration quantity\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub duration: Option<i64>,
    /// [Optional] Duration unit is `s`: seconds, `m`: minutes, `h`: hours, `d`: days, `t`: ticks\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub duration_unit: Option<DurationUnitEnum>,
    /// [Optional] The multiplier for non-binary options. E.g. lookbacks.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub multiplier: Option<f64>,
    /// [Optional] The tick that is predicted to have the highest/lowest value - for tickhigh and ticklow contracts.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub selected_tick: Option<i64>,
    /// Symbol code\n
    // Correct serde attribute construction - Use helper
    
    pub symbol: String,
}

