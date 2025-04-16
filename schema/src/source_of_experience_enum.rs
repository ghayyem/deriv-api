
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/new_account_maltainvest/send.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

/// How much knowledge and experience do you have in relation to online trading?
#[derive(Debug, Clone, Serialize, Deserialize)] // Enums should also derive Serialize/Deserialize
#[serde(rename_all = "snake_case")]
pub enum SourceOfExperienceEnum {
    I_Have_An_Academic_Degree_Professional_Certification_AndOr_Work_Experience,
    I_Trade_Forex_CFDs_And_Other_Complex_Financial_Instruments,
    I_Have_Attended_Seminars_Training_AndOr_Workshops,
    I_Have_Little_Experience,
    I_Have_No_Knowledge,
}

// Optional: Derive Default for enums, defaulting to the first variant? Or require explicit handling?
// For now, DO NOT derive Default for enums automatically. Structs needing them must handle it.

/* // Example: Deriving Default for Enum (use with caution)
impl Default for SourceOfExperienceEnum {
    fn default() -> Self {
        // Default to the first variant found
        Self::I_Have_An_Academic_Degree_Professional_Certification_AndOr_Work_Experience
    }
}
*/

