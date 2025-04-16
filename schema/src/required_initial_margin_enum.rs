
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/new_account_maltainvest/send.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

/// When would you be required to pay an initial margin?
#[derive(Debug, Clone, Serialize, Deserialize)] // Enums should also derive Serialize/Deserialize
#[serde(rename_all = "snake_case")]
pub enum RequiredInitialMarginEnum {
    When_Opening_A_Leveraged_CFD_Trade,
    When_Trading_Multipliers,
    When_Buying_Shares_Of_A_Company,
    All_Of_The_Above,
}

// Optional: Derive Default for enums, defaulting to the first variant? Or require explicit handling?
// For now, DO NOT derive Default for enums automatically. Structs needing them must handle it.

/* // Example: Deriving Default for Enum (use with caution)
impl Default for RequiredInitialMarginEnum {
    fn default() -> Self {
        // Default to the first variant found
        Self::When_Opening_A_Leveraged_CFD_Trade
    }
}
*/

