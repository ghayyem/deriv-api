
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_order_create/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

// It's a struct
/// For subscription requests only.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Subscription {
    /// A per-connection unique identifier. Can be passed to the `forget` API call to unsubscribe.\n
    // Correct serde attribute construction - Use helper
    
    pub id: String,
}

