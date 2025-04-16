
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/exchange_verification_code/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

// It's a struct
/// Exchange Verification Code details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExchangeVerificationCode {
    /// The new code generated after successful verification\n
    // Correct serde attribute construction - Use helper
    
    pub new_verification_code: String,
}

