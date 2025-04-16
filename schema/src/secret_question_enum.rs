
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/new_account_real/send.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

/// [Optional] Accept any value in enum list. Required for new account and existing client details will be used if client open another account.
#[derive(Debug, Clone, Serialize, Deserialize)] // Enums should also derive Serialize/Deserialize
#[serde(rename_all = "snake_case")]
pub enum SecretQuestionEnum {
    MotherS_Maiden_Name,
    Name_Of_Your_Pet,
    Name_Of_First_Love,
    Memorable_TownCity,
    Memorable_Date,
    Favourite_Dish,
    Brand_Of_First_Car,
    Favourite_Artist,
}

// Optional: Derive Default for enums, defaulting to the first variant? Or require explicit handling?
// For now, DO NOT derive Default for enums automatically. Structs needing them must handle it.

/* // Example: Deriving Default for Enum (use with caution)
impl Default for SecretQuestionEnum {
    fn default() -> Self {
        // Default to the first variant found
        Self::MotherS_Maiden_Name
    }
}
*/

