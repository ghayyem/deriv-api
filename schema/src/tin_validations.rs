
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/tin_validations/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::is_tin_mandatory_enum::IsTinMandatoryEnum; 

// It's a struct
/// Validations for Tax Identification Numbers (TIN)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TinValidations {
    /// Invalid regex patterns for tin validation\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub invalid_patterns: Option<Vec<String>>,
    /// Whether the TIN is mandatory for the selected country\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub is_tin_mandatory: Option<IsTinMandatoryEnum>,
    /// List of employment statuses that bypass TIN requirements for the selected country\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub tin_employment_status_bypass: Option<Vec<String>>,
    /// Country tax identifier formats.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub tin_format: Option<Vec<String>>,
}

