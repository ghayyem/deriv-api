
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_order_create/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::type_enum::TypeEnum; 
use crate::block_trade_enum::BlockTradeEnum; 

// It's a struct
/// Details of the advert for this order.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AdvertDetails {
    /// Indicates if this is block trade advert or not.\n
    // Correct serde attribute construction - Use helper
    
    pub block_trade: BlockTradeEnum,
    /// General information about the advert.\n
    // Correct serde attribute construction - Use helper
    
    pub description: String,
    /// The unique identifier for the advert.\n
    // Correct serde attribute construction - Use helper
    
    pub id: String,
    /// The payment method.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub payment_method: Option<String>,
    /// Type of the advert.\n
    // Correct serde attribute construction - Use helper
    #[serde(rename = "type")] 
    pub r#type: TypeEnum,
}

