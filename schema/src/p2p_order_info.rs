
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_order_info/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 
use serde_json::Value;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

// Import shared types from the *same* crate
use crate::status_enum::StatusEnum; 
use crate::type_enum::TypeEnum; 
use crate::dispute_details::DisputeDetails; 
use crate::is_seen_enum::IsSeenEnum; 
use crate::payment_method_details_value::PaymentMethodDetailsValue; 
use crate::advertiser_details::AdvertiserDetails; 
use crate::is_incoming_enum::IsIncomingEnum; 
use crate::verification_pending_enum::VerificationPendingEnum; 
use crate::review_details::ReviewDetails; 
use crate::client_details::ClientDetails; 
use crate::advert_details::AdvertDetails; 
use crate::is_reviewable_enum::IsReviewableEnum; 

// It's a struct
/// The information of P2P order.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct P2pOrderInfo {
    /// The currency of order.\n
    // Correct serde attribute construction - Use helper
    
    pub account_currency: String,
    /// Details of the advert for this order.\n
    // Correct serde attribute construction - Use helper
    
    pub advert_details: AdvertDetails,
    /// Details of the advertiser for this order.\n
    // Correct serde attribute construction - Use helper
    
    pub advertiser_details: AdvertiserDetails,
    /// The amount of the order.\n
    // Correct serde attribute construction - Use helper
    
    pub amount: f64,
    /// The amount of the order, formatted to appropriate decimal places.\n
    // Correct serde attribute construction - Use helper
    
    pub amount_display: f64,
    /// The URL to be used to initialise the chat for this order.\n
    // Correct serde attribute construction - Use helper
    
    pub chat_channel_url: String,
    /// Details of the client who created the order.\n
    // Correct serde attribute construction - Use helper
    
    pub client_details: ClientDetails,
    /// The epoch time of the order completion.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub completion_time: Option<DateTime<Utc>>,
    /// Seller contact information.\n
    // Correct serde attribute construction - Use helper
    
    pub contact_info: String,
    /// The epoch time of the order creation.\n
    // Correct serde attribute construction - Use helper
    
    pub created_time: DateTime<Utc>,
    /// Details of the order dispute.\n
    // Correct serde attribute construction - Use helper
    
    pub dispute_details: DisputeDetails,
    /// The epoch time in which the order will be expired.\n
    // Correct serde attribute construction - Use helper
    
    pub expiry_time: DateTime<Utc>,
    /// The unique identifier for this order.\n
    // Correct serde attribute construction - Use helper
    
    pub id: String,
    /// `1` if the order is created for the advert of the current client, otherwise `0`.\n
    // Correct serde attribute construction - Use helper
    
    pub is_incoming: IsIncomingEnum,
    /// `1` if a review can be given, otherwise `0`.\n
    // Correct serde attribute construction - Use helper
    
    pub is_reviewable: IsReviewableEnum,
    /// `1` if the latest order changes have been seen by the current client, otherwise `0`.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub is_seen: Option<IsSeenEnum>,
    /// Local currency for this order.\n
    // Correct serde attribute construction - Use helper
    
    pub local_currency: String,
    /// Payment instructions.\n
    // Correct serde attribute construction - Use helper
    
    pub payment_info: String,
    /// Supported payment methods. Comma separated list.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub payment_method: Option<String>,
    /// Details of available payment methods.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub payment_method_details: Option<HashMap<String, PaymentMethodDetailsValue>>,
    /// Names of supported payment methods.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub payment_method_names: Option<Vec<String>>,
    /// Cost in local currency.\n
    // Correct serde attribute construction - Use helper
    
    pub price: f64,
    /// Cost in local currency, formatted to appropriate decimal places.\n
    // Correct serde attribute construction - Use helper
    
    pub price_display: String,
    /// Conversion rate of the order.\n
    // Correct serde attribute construction - Use helper
    
    pub rate: f64,
    /// Conversion rate of the order, formatted to appropriate decimal places.\n
    // Correct serde attribute construction - Use helper
    
    pub rate_display: String,
    /// Details of the review you gave for this order, if any.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub review_details: Option<ReviewDetails>,
    /// Current order status.\n
    // Correct serde attribute construction - Use helper
    
    pub status: StatusEnum,
    /// Whether this is a buy or a sell.\n
    // Correct serde attribute construction - Use helper
    #[serde(rename = "type")] 
    pub r#type: TypeEnum,
    /// If blocked for too many failed verification attempts, the epoch time that the block will end.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub verification_lockout_until: Option<DateTime<Utc>>,
    /// If a verification request has already been made, the epoch time that another verification request can be made.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub verification_next_request: Option<DateTime<Utc>>,
    /// Indicates that the seller in the process of confirming the order.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub verification_pending: Option<VerificationPendingEnum>,
    /// Epoch time that the current verification token will expire.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub verification_token_expiry: Option<i64>,
}

