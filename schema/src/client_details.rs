
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_order_create/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::is_online_enum::IsOnlineEnum; 

// It's a struct
/// Details of the client who created the order.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ClientDetails {
    /// The client's first name.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub first_name: Option<String>,
    /// The client's unique P2P identifier.\n
    // Correct serde attribute construction - Use helper
    
    pub id: String,
    /// Indicates if the advertiser is currently online.\n
    // Correct serde attribute construction - Use helper
    
    pub is_online: IsOnlineEnum,
    /// The client's last name.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub last_name: Option<String>,
    /// Epoch of the latest time the advertiser was online, up to 6 months.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub last_online_time: Option<i64>,
    /// The client's account identifier.\n
    // Correct serde attribute construction - Use helper
    
    pub loginid: String,
    /// The client's displayed name.\n
    // Correct serde attribute construction - Use helper
    
    pub name: String,
}

