
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/website_status/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::max_proposal_subscription::MaxProposalSubscription; 
use crate::max_requestes_general::MaxRequestesGeneral; 
use crate::max_requests_pricing::MaxRequestsPricing; 
use crate::max_requests_outcome::MaxRequestsOutcome; 

// It's a struct
/// Maximum number of API calls during specified period of time.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ApiCallLimits {
    /// Maximum subscription to proposal calls.\n
    // Correct serde attribute construction - Use helper
    
    pub max_proposal_subscription: MaxProposalSubscription,
    /// Maximum number of general requests allowed during specified period of time.\n
    // Correct serde attribute construction - Use helper
    
    pub max_requestes_general: MaxRequestesGeneral,
    /// Maximum number of outcome requests allowed during specified period of time.\n
    // Correct serde attribute construction - Use helper
    
    pub max_requests_outcome: MaxRequestsOutcome,
    /// Maximum number of pricing requests allowed during specified period of time.\n
    // Correct serde attribute construction - Use helper
    
    pub max_requests_pricing: MaxRequestsPricing,
}

