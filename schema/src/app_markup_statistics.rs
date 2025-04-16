
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/app_markup_statistics/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::breakdown_item::BreakdownItem; 

// It's a struct
/// App Markup transaction statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AppMarkupStatistics {
    /// Array of summed app markups grouped by app_id\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub breakdown: Option<Vec<BreakdownItem>>,
    /// The sum of markup the client paid in USD\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub total_app_markup_usd: Option<f64>,
    /// The total count of transactions\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub total_transactions_count: Option<f64>,
}

