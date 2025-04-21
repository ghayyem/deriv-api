
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/trading_servers/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::geolocation::Geolocation; 
use crate::id::Id; 
use crate::recommended::Recommended; 
use crate::environment::Environment; 
use crate::account_type::AccountType; 
use crate::disabled::Disabled; 

// It's a struct
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TradingServerItem {
    /// Supported trading account type.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub account_type: Option<AccountType>,
    /// Flag to represent if this server is currently disabled or not\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub disabled: Option<Disabled>,
    /// Current environment (installation instance) where servers are deployed. Currently, there are one demo and two real environments.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub environment: Option<Environment>,
    /// Object containing geolocation information of the server.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub geolocation: Option<Geolocation>,
    /// Server unique id.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub id: Option<Id>,
    /// Market type\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub market_type: Option<String>,
    /// Error message to client when server is disabled\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub message_to_client: Option<String>,
    /// Flag to represent if this is server is recommended based on client's country of residence.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub recommended: Option<Recommended>,
    /// Account type supported by the server.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub supported_accounts: Option<Vec<String>>,
}

