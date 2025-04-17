
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/api_token/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::new_token_enum::NewTokenEnum; 
use crate::tokens_item::TokensItem; 
use crate::delete_token_enum::DeleteTokenEnum; 

// It's a struct
/// Contains the result of API token according to the type of request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ApiToken {
    /// Token deleted.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub delete_token: Option<DeleteTokenEnum>,
    /// Token created.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub new_token: Option<NewTokenEnum>,
    /// API tokens\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub tokens: Option<Vec<TokensItem>>,
}

