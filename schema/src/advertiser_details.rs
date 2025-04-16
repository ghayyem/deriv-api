
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_order_create/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::is_online_enum::IsOnlineEnum; 
use crate::is_recommended_enum::IsRecommendedEnum; 

// It's a struct
/// Details of the advertiser for this order.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AdvertiserDetails {
    /// The advertiser's first name.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub first_name: Option<String>,
    /// The advertiser's unique identifier.\n
    // Correct serde attribute construction - Use helper
    
    pub id: String,
    /// Indicates if the advertiser is currently online.\n
    // Correct serde attribute construction - Use helper
    
    pub is_online: IsOnlineEnum,
    /// Optional field, indicates if the advertiser is recommended.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub is_recommended: Option<IsRecommendedEnum>,
    /// The advertiser's last name.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub last_name: Option<String>,
    /// Epoch of the latest time the advertiser was online, up to 6 months.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub last_online_time: Option<i64>,
    /// The advertiser's account identifier.\n
    // Correct serde attribute construction - Use helper
    
    pub loginid: String,
    /// The advertiser's displayed name.\n
    // Correct serde attribute construction - Use helper
    
    pub name: String,
}

