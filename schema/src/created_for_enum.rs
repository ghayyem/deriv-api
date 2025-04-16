
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/confirm_email/send.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

/// [Optional] Purpose of the email verification. If set to 'account_opening', the API will only return the verification response without updating the user's email verification status.
#[derive(Debug, Clone, Serialize, Deserialize)] // Enums should also derive Serialize/Deserialize
#[serde(rename_all = "snake_case")]
pub enum CreatedForEnum {
    AccountVerification,
    AccountOpening,
}

// Optional: Derive Default for enums, defaulting to the first variant? Or require explicit handling?
// For now, DO NOT derive Default for enums automatically. Structs needing them must handle it.

/* // Example: Deriving Default for Enum (use with caution)
impl Default for CreatedForEnum {
    fn default() -> Self {
        // Default to the first variant found
        Self::AccountVerification
    }
}
*/

