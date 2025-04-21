
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/contracts_for_company/send.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

/// [Optional] Indicates which landing company to get a list of contracts for. If you are logged in, your account's landing company will override this field.
#[derive(Debug, Clone, Serialize, Deserialize)] // Enums should also derive Serialize/Deserialize
#[serde(rename_all = "snake_case")]
pub enum LandingCompany {
    Iom,
    Malta,
    Maltainvest,
    Svg,
    Virtual,
    Vanuatu,
}

// Optional: Derive Default for enums, defaulting to the first variant? Or require explicit handling?
// For now, DO NOT derive Default for enums automatically. Structs needing them must handle it.

/* // Example: Deriving Default for Enum (use with caution)
impl Default for LandingCompany {
    fn default() -> Self {
        // Default to the first variant found
        Self::Iom
    }
}
*/

