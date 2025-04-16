
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_order_confirm/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::dry_run_enum::DryRunEnum; 
use crate::status_enum::StatusEnum; 

// It's a struct
/// Confirmation details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct P2pOrderConfirm {
    /// The `dry_run` was successful.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub dry_run: Option<DryRunEnum>,
    /// The unique identifier for the order.\n
    // Correct serde attribute construction - Use helper
    
    pub id: String,
    /// The new status of the order.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub status: Option<StatusEnum>,
}

