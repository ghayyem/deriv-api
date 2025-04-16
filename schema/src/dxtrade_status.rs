
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/website_status/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

// It's a struct
/// Suspension status of Dxtrade/DerivX API calls
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DxtradeStatus {
    /// Suspension of Dxtrade/DerivX API calls on all servers.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub all: Option<i64>,
    /// Suspension of Dxtrade/DerivX API calls on demo servers.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub demo: Option<i64>,
    /// Suspension of Dxtrade/DerivX API calls on real trading servers.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub real: Option<i64>,
}

