
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/residence_list/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 
use serde_json::Value;



// Import shared types from the *same* crate

// It's a struct
/// Tax configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Tax {
    /// Mandatory TIN flag\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub mandatory: Option<bool>,
    /// POI equivalent flag\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub poi_equivalent: Option<bool>,
    /// Cleanup regex\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub tin_cleaner: Option<String>,
    /// Country tax identifier format\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub tin_format: Option<Vec<String>>,
    /// Description of the TIN format\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub tin_format_description: Option<Value>,
}

