
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/partner_accounts/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 

use std::collections::HashMap;


// Import shared types from the *same* crate
use crate::partner_settings_item::PartnerSettingsItem; 

// It's a struct
/// Partner Accounts against a user
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PartnerAccounts {
    /// List of partner_settigns for all accounts associated as partners.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub partner_settings: Option<Vec<PartnerSettingsItem>>,
    /// Platform URL for Dynamic works dashboard to be redirected from Partners Hub which will be set in BackOffice.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub platform_url: Option<HashMap<String, f64>>,
}

