
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/contracts_for_company/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

// It's a struct
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AvailableItem {
    /// Category of contract barrier.\n
    // Correct serde attribute construction - Use helper
    
    pub barrier_category: String,
    /// Category of contract.\n
    // Correct serde attribute construction - Use helper
    
    pub contract_category: String,
    /// Display name for the contract category, localized to selected language.\n
    // Correct serde attribute construction - Use helper
    
    pub contract_category_display: String,
    /// Display name for the contract, localized to selected language.\n
    // Correct serde attribute construction - Use helper
    
    pub contract_display: String,
    /// Type of contract.\n
    // Correct serde attribute construction - Use helper
    
    pub contract_type: String,
    /// Default stake for the contract\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub default_stake: Option<f64>,
    /// Type of sentiment.\n
    // Correct serde attribute construction - Use helper
    
    pub sentiment: String,
}

