
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_order_review/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 


use chrono::{DateTime, Utc};

// Import shared types from the *same* crate
use crate::recommended_enum::RecommendedEnum; 

// It's a struct
/// Details of the created order review.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct P2pOrderReview {
    /// The reviewed advertiser's identification number.\n
    // Correct serde attribute construction - Use helper
    
    pub advertiser_id: String,
    /// The epoch time of the review.\n
    // Correct serde attribute construction - Use helper
    
    pub created_time: DateTime<Utc>,
    /// The order identification number.\n
    // Correct serde attribute construction - Use helper
    
    pub order_id: String,
    /// Rating for the transaction, 1 to 5.\n
    // Correct serde attribute construction - Use helper
    
    pub rating: i64,
    /// `1` if the advertiser is recommended, `0` if not recommended.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub recommended: Option<RecommendedEnum>,
}

