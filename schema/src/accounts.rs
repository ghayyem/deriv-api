
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/transfer_between_accounts/send.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

/// [Optional] To control the list of accounts returned when `account_from` or `account_to` is not provided. `brief` (default value) means that accounts with `mt5` account_type will be excluded; it will run faster. `all` means that all accounts with any account_type (including `mt5`) will be returned.
#[derive(Debug, Clone, Serialize, Deserialize)] // Enums should also derive Serialize/Deserialize
#[serde(rename_all = "snake_case")]
pub enum Accounts {
    All,
    Brief,
}

// Optional: Derive Default for enums, defaulting to the first variant? Or require explicit handling?
// For now, DO NOT derive Default for enums automatically. Structs needing them must handle it.

/* // Example: Deriving Default for Enum (use with caution)
impl Default for Accounts {
    fn default() -> Self {
        // Default to the first variant found
        Self::All
    }
}
*/

