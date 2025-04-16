
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/get_settings/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::verified_enum::VerifiedEnum; 

// It's a struct
/// The status of the Phone Number Verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PhoneNumberVerification {
    /// Indicates the attempts remaining for /phone_number_challenge\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub challenge_attempts_remaining: Option<i64>,
    /// (Optional) Indicates the timestamp for the next verification attempt\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub next_attempt: Option<i64>,
    /// (Optional) Indicates the timestamp for the next email verification attempt\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub next_email_attempt: Option<i64>,
    /// (Optional) Indicates the timestamp for the next verify attempt\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub next_verify_attempt: Option<i64>,
    /// (Optional) Indicates the timestamp for the current's session email code expiry\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub session_timestamp: Option<i64>,
    /// Indicates the verification status of the client's phone number.\n
    // Correct serde attribute construction - Use helper
    
    pub verified: VerifiedEnum,
    /// Indicates the attempts remaining for /phone_number_verification\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub verify_attempts_remaining: Option<i64>,
}

