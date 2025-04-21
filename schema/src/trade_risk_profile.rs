
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/proposal/send.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

/// [Only for Snowball] The trade risk profile for the Snowball contract. Higher risk profile offers higher coupon rate at the expense of higher probability of breaching caution price
#[derive(Debug, Clone, Serialize, Deserialize)] // Enums should also derive Serialize/Deserialize
#[serde(rename_all = "snake_case")]
pub enum TradeRiskProfile {
    Low,
    Medium,
    High,
}

// Optional: Derive Default for enums, defaulting to the first variant? Or require explicit handling?
// For now, DO NOT derive Default for enums automatically. Structs needing them must handle it.

/* // Example: Deriving Default for Enum (use with caution)
impl Default for TradeRiskProfile {
    fn default() -> Self {
        // Default to the first variant found
        Self::Low
    }
}
*/

