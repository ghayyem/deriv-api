
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_chat_create/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate

// It's a struct
/// Information of the P2P chat.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct P2pChatCreate {
    /// The URL to be used to initialise the chat for the requested order.\n
    // Correct serde attribute construction - Use helper
    
    pub channel_url: String,
    /// The unique identifier for the order that the chat belongs to.\n
    // Correct serde attribute construction - Use helper
    
    pub order_id: String,
}

