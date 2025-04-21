
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/new_account_maltainvest/send.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

/// How much experience do you have with other financial instruments?
#[derive(Debug, Clone, Serialize, Deserialize)] // Enums should also derive Serialize/Deserialize
#[serde(rename_all = "snake_case")]
pub enum TradingExperienceFinancialInstruments {
    No_Experience,
    Less_Than_A_Year,
    _1__2_Years,
    Over_3_Years,
}

// Optional: Derive Default for enums, defaulting to the first variant? Or require explicit handling?
// For now, DO NOT derive Default for enums automatically. Structs needing them must handle it.

/* // Example: Deriving Default for Enum (use with caution)
impl Default for TradingExperienceFinancialInstruments {
    fn default() -> Self {
        // Default to the first variant found
        Self::No_Experience
    }
}
*/

