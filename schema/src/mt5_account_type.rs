
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/mt5_new_account/send.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

/// [Optional] Financial: Variable spreads, High leverage. Financial STP: Variable spreads, Medium Leverage, more products. If 'account_type' set to 'financial', setting 'mt5_account_type' is also required.
#[derive(Debug, Clone, Serialize, Deserialize)] // Enums should also derive Serialize/Deserialize
#[serde(rename_all = "snake_case")]
pub enum Mt5AccountType {
    Financial,
    FinancialStp,
    Gold,
}

// Optional: Derive Default for enums, defaulting to the first variant? Or require explicit handling?
// For now, DO NOT derive Default for enums automatically. Structs needing them must handle it.

/* // Example: Deriving Default for Enum (use with caution)
impl Default for Mt5AccountType {
    fn default() -> Self {
        // Default to the first variant found
        Self::Financial
    }
}
*/

