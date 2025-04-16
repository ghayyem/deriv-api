
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/crypto_estimations/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 


use chrono::{DateTime, Utc};

// Import shared types from the *same* crate

// It's a struct
/// Estimated fee for crypto withdrawal calculated based on the current network conditions.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WithdrawalFee {
    /// Expiry time for the estimated fee in epoch.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub expiry_time: Option<DateTime<Utc>>,
    /// Unique identifier for the estimated fee which allows locking the fee for a client.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub unique_id: Option<String>,
    /// Value of current estimated fee.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub value: Option<f64>,
}

