
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/contracts_for_company/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::available_item::AvailableItem; 

// It's a struct
/// List of available contracts for a given landing company.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ContractsForCompany {
    /// List of available contracts.\n
    // Correct serde attribute construction - Use helper
    
    pub available: Vec<AvailableItem>,
    /// Count of contracts available\n
    // Correct serde attribute construction - Use helper
    
    pub hit_count: f64,
}

