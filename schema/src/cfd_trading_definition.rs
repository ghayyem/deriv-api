
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/new_account_maltainvest/send.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

/// In your understanding, CFD trading allows you to:
#[derive(Debug, Clone, Serialize, Deserialize)] // Enums should also derive Serialize/Deserialize
#[serde(rename_all = "snake_case")]
pub enum CfdTradingDefinition {
    Purchase_Shares_Of_A_Company_Or_Physical_Commodities,
    Place_A_Bet_On_The_Price_Movement,
    Speculate_On_The_Price_Movement,
    Make_A_Long_Term_Investment,
}

// Optional: Derive Default for enums, defaulting to the first variant? Or require explicit handling?
// For now, DO NOT derive Default for enums automatically. Structs needing them must handle it.

/* // Example: Deriving Default for Enum (use with caution)
impl Default for CfdTradingDefinition {
    fn default() -> Self {
        // Default to the first variant found
        Self::Purchase_Shares_Of_A_Company_Or_Physical_Commodities
    }
}
*/

