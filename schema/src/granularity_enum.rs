
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/ticks_history/send.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

/// [Optional] Only applicable for style: `candles`. Candle time-dimension width setting. (default: `60`).
#[derive(Debug, Clone, Serialize, Deserialize)] // Enums should also derive Serialize/Deserialize
#[serde(rename_all = "snake_case")]
pub enum GranularityEnum {
    Value60 = 60,
    Value120 = 120,
    Value180 = 180,
    Value300 = 300,
    Value600 = 600,
    Value900 = 900,
    Value1800 = 1800,
    Value3600 = 3600,
    Value7200 = 7200,
    Value14400 = 14400,
    Value28800 = 28800,
    Value86400 = 86400,
}

// Optional: Derive Default for enums, defaulting to the first variant? Or require explicit handling?
// For now, DO NOT derive Default for enums automatically. Structs needing them must handle it.

/* // Example: Deriving Default for Enum (use with caution)
impl Default for GranularityEnum {
    fn default() -> Self {
        // Default to the first variant found
        Self::Value60
    }
}
*/

