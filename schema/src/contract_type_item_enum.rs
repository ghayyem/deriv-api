
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/active_symbols/send.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

/// 
#[derive(Debug, Clone, Serialize, Deserialize)] // Enums should also derive Serialize/Deserialize
#[serde(rename_all = "snake_case")]
pub enum ContractTypeItemEnum {
    MULTUP,
    MULTDOWN,
    UPORDOWN,
    EXPIRYRANGE,
    ONETOUCH,
    CALLE,
    LBHIGHLOW,
    ASIAND,
    EXPIRYRANGEE,
    DIGITDIFF,
    DIGITMATCH,
    DIGITOVER,
    PUTE,
    DIGITUNDER,
    NOTOUCH,
    CALL,
    RANGE,
    LBFLOATPUT,
    DIGITODD,
    PUT,
    ASIANU,
    LBFLOATCALL,
    EXPIRYMISSE,
    EXPIRYMISS,
    DIGITEVEN,
    TICKHIGH,
    TICKLOW,
    RESETCALL,
    RESETPUT,
    CALLSPREAD,
    PUTSPREAD,
    RUNHIGH,
    RUNLOW,
    ACCU,
    VANILLALONGCALL,
    VANILLALONGPUT,
    TURBOSLONG,
    TURBOSSHORT,
}

// Optional: Derive Default for enums, defaulting to the first variant? Or require explicit handling?
// For now, DO NOT derive Default for enums automatically. Structs needing them must handle it.

/* // Example: Deriving Default for Enum (use with caution)
impl Default for ContractTypeItemEnum {
    fn default() -> Self {
        // Default to the first variant found
        Self::MULTUP
    }
}
*/

