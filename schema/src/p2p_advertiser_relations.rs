
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_advertiser_relations/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::blocked_advertisers_item::BlockedAdvertisersItem; 
use crate::favourite_advertisers_item::FavouriteAdvertisersItem; 

// It's a struct
/// P2P advertiser relations information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct P2pAdvertiserRelations {
    /// List of advertisers blocked by the current user.\n
    // Correct serde attribute construction - Use helper
    
    pub blocked_advertisers: Vec<BlockedAdvertisersItem>,
    /// Favourite advertisers of the current user.\n
    // Correct serde attribute construction - Use helper
    
    pub favourite_advertisers: Vec<FavouriteAdvertisersItem>,
}

