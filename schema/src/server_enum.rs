
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/mt5_new_account/send.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

/// [Optional] Trade server.
#[derive(Debug, Clone, Serialize, Deserialize)] // Enums should also derive Serialize/Deserialize
#[serde(rename_all = "snake_case")]
pub enum ServerEnum {
    P01Ts01,
    P01Ts02,
    P01Ts03,
    P01Ts04,
    P02Ts02,
    P03Ts01,
    P03Ts02,
}

// Optional: Derive Default for enums, defaulting to the first variant? Or require explicit handling?
// For now, DO NOT derive Default for enums automatically. Structs needing them must handle it.

/* // Example: Deriving Default for Enum (use with caution)
impl Default for ServerEnum {
    fn default() -> Self {
        // Default to the first variant found
        Self::P01Ts01
    }
}
*/

