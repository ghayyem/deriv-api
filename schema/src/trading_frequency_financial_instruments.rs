
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/new_account_maltainvest/send.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

/// How many trades have you placed with other financial instruments in the past 12 months?
#[derive(Debug, Clone, Serialize, Deserialize)] // Enums should also derive Serialize/Deserialize
#[serde(rename_all = "snake_case")]
pub enum TradingFrequencyFinancialInstruments {
    No_Transactions_In_The_Past_12_Months,
    _1__5_Transactions_In_The_Past_12_Months,
    _6__10_Transactions_In_The_Past_12_Months,
    _11__39_Transactions_In_The_Past_12_Months,
    _40_Transactions_Or_More_In_The_Past_12_Months,
}

// Optional: Derive Default for enums, defaulting to the first variant? Or require explicit handling?
// For now, DO NOT derive Default for enums automatically. Structs needing them must handle it.

/* // Example: Deriving Default for Enum (use with caution)
impl Default for TradingFrequencyFinancialInstruments {
    fn default() -> Self {
        // Default to the first variant found
        Self::No_Transactions_In_The_Past_12_Months
    }
}
*/

