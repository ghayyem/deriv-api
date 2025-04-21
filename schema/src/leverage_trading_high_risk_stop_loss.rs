
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/new_account_maltainvest/send.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

/// Leverage trading is high-risk, so it's a good idea to use risk management features such as stop loss. Stop loss allows you to
#[derive(Debug, Clone, Serialize, Deserialize)] // Enums should also derive Serialize/Deserialize
#[serde(rename_all = "snake_case")]
pub enum LeverageTradingHighRiskStopLoss {
    Cancel_Your_Trade_At_Any_Time_Within_A_Chosen_Timeframe,
    Close_Your_Trade_Automatically_When_The_Loss_Is_More_Than_Or_Equal_To_A_Specific_Amount,
    Close_Your_Trade_Automatically_When_The_Profit_Is_More_Than_Or_Equal_To_A_Specific_Amount,
    Make_A_Guaranteed_Profit_On_Your_Trade,
}

// Optional: Derive Default for enums, defaulting to the first variant? Or require explicit handling?
// For now, DO NOT derive Default for enums automatically. Structs needing them must handle it.

/* // Example: Deriving Default for Enum (use with caution)
impl Default for LeverageTradingHighRiskStopLoss {
    fn default() -> Self {
        // Default to the first variant found
        Self::Cancel_Your_Trade_At_Any_Time_Within_A_Chosen_Timeframe
    }
}
*/

