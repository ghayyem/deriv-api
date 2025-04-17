
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/balance/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::deriv::Deriv; 
use crate::mt5_demo::Mt5Demo; 
use crate::mt5::Mt5; 
use crate::deriv_demo::DerivDemo; 

// It's a struct
/// Summary totals of accounts by type.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Total {
    /// Total balance of all real money Deriv accounts.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub deriv: Option<Deriv>,
    /// Total balance of all demo Deriv accounts.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub deriv_demo: Option<DerivDemo>,
    /// Total balance of all MT5 real money accounts.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub mt5: Option<Mt5>,
    /// Total balance of all MT5 demo accounts.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub mt5_demo: Option<Mt5Demo>,
}

