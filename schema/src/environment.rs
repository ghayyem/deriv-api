
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/mt5_login_list/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

/// The environment. E.g. Deriv-Server.
#[derive(Debug, Clone, Serialize, Deserialize)] // Enums should also derive Serialize/Deserialize
#[serde(rename_all = "snake_case")]
pub enum Environment {
    Deriv_Demo,
    Deriv_Server,
    Deriv_Server_02,
    Deriv_Server_03,
    DerivFX_Server,
    DerivFX_Server_02,
    DerivFX_Server_03,
    DerivVU_Server,
    DerivVU_Server_02,
    DerivVU_Server_03,
    DerivSVG_Server,
    DerivSVG_Server_02,
    DerivSVG_Server_03,
    DerivMT_Server,
    DerivMT_Server_02,
    DerivMT_Server_03,
    DerivBVI_Server,
    DerivBVI_Server_02,
    DerivBVI_Server_03,
    DerivMU_Server,
    DerivMU_Server_02,
    DerivMU_Server_03,
}

// Optional: Derive Default for enums, defaulting to the first variant? Or require explicit handling?
// For now, DO NOT derive Default for enums automatically. Structs needing them must handle it.

/* // Example: Deriving Default for Enum (use with caution)
impl Default for Environment {
    fn default() -> Self {
        // Default to the first variant found
        Self::Deriv_Demo
    }
}
*/

