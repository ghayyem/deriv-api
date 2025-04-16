
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_order_list/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 


use chrono::{DateTime, Utc};

// Import shared types from the *same* crate
use crate::recommended_enum::RecommendedEnum; 

// It's a struct
/// Details of the review you gave for this order, if any.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReviewDetails {
    /// The epoch time of the review.\n
    // Correct serde attribute construction - Use helper
    
    pub created_time: DateTime<Utc>,
    /// Rating for the transaction, 1 to 5.\n
    // Correct serde attribute construction - Use helper
    
    pub rating: i64,
    /// `1` if the advertiser is recommended, `0` if not recommended.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub recommended: Option<RecommendedEnum>,
}

