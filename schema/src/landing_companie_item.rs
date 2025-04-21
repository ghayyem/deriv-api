
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/kyc_auth_status/send.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

/// 
#[derive(Debug, Clone, Serialize, Deserialize)] // Enums should also derive Serialize/Deserialize
#[serde(rename_all = "snake_case")]
pub enum LandingCompanieItem {
    Iom,
    Malta,
    Maltainvest,
    Svg,
    Virtual,
    Vanuatu,
    Labuan,
    Samoa,
    Samoa_Virtual,
    Bvi,
    Dsl,
}

// Optional: Derive Default for enums, defaulting to the first variant? Or require explicit handling?
// For now, DO NOT derive Default for enums automatically. Structs needing them must handle it.

/* // Example: Deriving Default for Enum (use with caution)
impl Default for LandingCompanieItem {
    fn default() -> Self {
        // Default to the first variant found
        Self::Iom
    }
}
*/

