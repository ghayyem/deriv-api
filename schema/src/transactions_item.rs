
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/statement/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::to::To; 
use crate::from::From; 
use crate::action_type_enum::ActionTypeEnum; 
use crate::fees::Fees; 

// It's a struct
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TransactionsItem {
    /// It is the type of action.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub action_type: Option<ActionTypeEnum>,
    /// It is the amount of transaction.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub amount: Option<f64>,
    /// ID of the application where this contract was purchased.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub app_id: Option<i64>,
    /// It is the remaining balance.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub balance_after: Option<f64>,
    /// It is the contract ID.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub contract_id: Option<i64>,
    /// Contains details about fees used for transfer. It is present only when action type is transfer.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub fees: Option<Fees>,
    /// Contains details of account from which amount was transferred. It is present only when action type is transfer.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub from: Option<From>,
    /// The description of contract purchased if description is set to `1`.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub longcode: Option<String>,
    /// Payout price\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub payout: Option<f64>,
    /// Time at which contract was purchased, present only for sell transaction\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub purchase_time: Option<i64>,
    /// Internal transaction identifier for the corresponding buy transaction ( set only for contract selling )\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub reference_id: Option<i64>,
    /// Compact description of the contract purchased if description is set to `1`.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub shortcode: Option<String>,
    /// Contains details of account to which amount was transferred. It is present only when action type is transfer.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub to: Option<To>,
    /// It is the transaction ID. In statement every contract (buy or sell) and every payment has a unique ID.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub transaction_id: Option<i64>,
    /// It is the time of transaction.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub transaction_time: Option<i64>,
    /// Additional withdrawal details such as typical processing times, if description is set to `1`.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub withdrawal_details: Option<String>,
}

