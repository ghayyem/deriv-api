
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_advertiser_update/send.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

// It's a struct
/// Periods of availabily. A null value will clear the existing schedule.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ScheduleItem {
    /// Minute of week when availablility ends. Zero is Sunday 00:00 UST.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub end_min: Option<i64>,
    /// Minute of week when availablility starts. Zero is Sunday 00:00 UST.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub start_min: Option<i64>,
}

