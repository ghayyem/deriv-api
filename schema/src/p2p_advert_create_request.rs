
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_advert_create/send.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


// Import required types from the *same* crate
use crate::rate_type_enum::RateTypeEnum;
use crate::type_enum::TypeEnum;
use crate::block_trade_enum::BlockTradeEnum;

/// Creates a P2P (Peer to Peer) advert. Can only be used by an approved P2P advertiser.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct P2pAdvertCreateRequest {
    /// The total amount of the advert, in advertiser's account currency.\n
    // Correct serde attribute construction - Use helper
    
    pub amount: f64,
    /// [Optional] Indicates if this is block trade ad or not. Default: 0.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub block_trade: Option<BlockTradeEnum>,
    /// [Optional] Advertiser contact information.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub contact_info: Option<String>,
    /// [Optional] General information about the advert.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub description: Option<String>,
    /// [Optional] 2 letter country codes. Counterparties who do not live in these countries will not be allowed to place orders against the advert.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub eligible_countries: Option<Vec<String>>,
    /// [Optional] Local currency for this advert. If not provided, will use the currency of client's residence by default.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub local_currency: Option<String>,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub loginid: Option<String>,
    /// Maximum allowed amount for the orders of this advert, in advertiser's `account_currency`. Should be more than or equal to `min_order_amount`\n
    // Correct serde attribute construction - Use helper
    
    pub max_order_amount: f64,
    /// [Optional] Counterparties who have a 30 day completion rate less than this value will not be allowed to place orders against the advert.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub min_completion_rate: Option<f64>,
    /// [Optional] Counterparties who joined less than this number of days ago will not be allowed to place orders against the advert.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub min_join_days: Option<i64>,
    /// Minimum allowed amount for the orders of this advert, in advertiser's `account_currency`. Should be less than or equal to `max_order_amount`.\n
    // Correct serde attribute construction - Use helper
    
    pub min_order_amount: f64,
    /// [Optional] Counterparties who have an average rating less than this value will not be allowed to place orders against the advert.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub min_rating: Option<f64>,
    /// [Optional] Expiry period (seconds) for order created against this ad.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub order_expiry_period: Option<i64>,
    /// Field 'p2p_advert_create' mapped to Value due to complexity/potential issues.\n
    // Correct serde attribute construction - Use helper
    
    pub p2p_advert_create: Value,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub passthrough: Option<Value>,
    /// [Optional] Payment instructions.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub payment_info: Option<String>,
    /// [Optional] Payment method name (deprecated).\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub payment_method: Option<String>,
    /// IDs of previously saved payment methods as returned from p2p_advertiser_payment_methods, only applicable for sell ads.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub payment_method_ids: Option<Vec<i64>>,
    /// Payment method identifiers as returned from p2p_payment_methods, only applicable for buy ads.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub payment_method_names: Option<Vec<String>>,
    /// Conversion rate from advertiser's account currency to `local_currency`. An absolute rate value (fixed), or percentage offset from current market rate (floating).\n
    // Correct serde attribute construction - Use helper
    
    pub rate: f64,
    /// Type of rate, fixed or floating.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub rate_type: Option<RateTypeEnum>,
    /// [Optional] Used to map request to response.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub req_id: Option<i64>,
    /// The advertisement represents the intention to perform this action on your Deriv account funds.\n
    // Correct serde attribute construction - Use helper
    #[serde(rename = "type")] 
    pub r#type: TypeEnum,
}

