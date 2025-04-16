
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/contract_update/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 


use chrono::{DateTime, Utc};

// Import shared types from the *same* crate

// It's a struct
/// The target spot price where the contract will be closed automatically at the loss specified by the user.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StopLoss {
    /// Localized display name\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub display_name: Option<String>,
    /// Stop loss amount\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub order_amount: Option<f64>,
    /// Stop loss order epoch\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub order_date: Option<DateTime<Utc>>,
    /// Stop loss pip-sized barrier value\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub value: Option<f64>,
}

