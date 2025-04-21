
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/transfer_between_accounts/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

/// Sub account type
#[derive(Debug, Clone, Serialize, Deserialize)] // Enums should also derive Serialize/Deserialize
#[serde(rename_all = "snake_case")]
pub enum SubAccountType {
    Financial,
    FinancialStp,
    Standard,
    SwapFree,
    ZeroSpread,
    Gold,
}

// Optional: Derive Default for enums, defaulting to the first variant? Or require explicit handling?
// For now, DO NOT derive Default for enums automatically. Structs needing them must handle it.

/* // Example: Deriving Default for Enum (use with caution)
impl Default for SubAccountType {
    fn default() -> Self {
        // Default to the first variant found
        Self::Financial
    }
}
*/

