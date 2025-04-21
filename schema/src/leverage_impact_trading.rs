
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/new_account_maltainvest/send.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

/// How does leverage affect CFD trading?
#[derive(Debug, Clone, Serialize, Deserialize)] // Enums should also derive Serialize/Deserialize
#[serde(rename_all = "snake_case")]
pub enum LeverageImpactTrading {
    Leverage_Is_A_Risk_Mitigation_Technique,
    Leverage_Prevents_You_From_Opening_Large_Positions,
    Leverage_Guarantees_Profits,
    Leverage_Lets_You_Open_Larger_Positions_For_A_Fraction_Of_The_TradeS_Value,
}

// Optional: Derive Default for enums, defaulting to the first variant? Or require explicit handling?
// For now, DO NOT derive Default for enums automatically. Structs needing them must handle it.

/* // Example: Deriving Default for Enum (use with caution)
impl Default for LeverageImpactTrading {
    fn default() -> Self {
        // Default to the first variant found
        Self::Leverage_Is_A_Risk_Mitigation_Technique
    }
}
*/

