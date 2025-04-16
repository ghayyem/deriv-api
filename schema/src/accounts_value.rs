
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/balance/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::demo_account_enum::DemoAccountEnum; 
use crate::status_enum::StatusEnum; 
use crate::type_enum::TypeEnum; 

// It's a struct
/// Individual accounts details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AccountsValue {
    /// Account balance\n
    // Correct serde attribute construction - Use helper
    
    pub balance: f64,
    /// Account balance converted the total currency.\n
    // Correct serde attribute construction - Use helper
    
    pub converted_amount: f64,
    /// Account currency.\n
    // Correct serde attribute construction - Use helper
    
    pub currency: String,
    /// If set to 1, this is a demo account.\n
    // Correct serde attribute construction - Use helper
    
    pub demo_account: DemoAccountEnum,
    /// Boolean value of 1 or 0. Indicates the status of account. 1 indicates account is good and accessible.\n
    // Correct serde attribute construction - Use helper
    
    pub status: StatusEnum,
    /// Type of account.\n
    // Correct serde attribute construction - Use helper
    #[serde(rename = "type")] 
    pub r#type: TypeEnum,
}

