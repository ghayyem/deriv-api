
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/buy_contract_for_multiple_accounts/send.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

/// A valid contract-type
#[derive(Debug, Clone, Serialize, Deserialize)] // Enums should also derive Serialize/Deserialize
#[serde(rename_all = "snake_case")]
pub enum ContractTypeEnum {
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
    VANILLALONGCALL,
    VANILLALONGPUT,
    TURBOSLONG,
    TURBOSSHORT,
}

// Optional: Derive Default for enums, defaulting to the first variant? Or require explicit handling?
// For now, DO NOT derive Default for enums automatically. Structs needing them must handle it.

/* // Example: Deriving Default for Enum (use with caution)
impl Default for ContractTypeEnum {
    fn default() -> Self {
        // Default to the first variant found
        Self::MULTUP
    }
}
*/

