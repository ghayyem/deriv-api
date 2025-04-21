
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/residence_list/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::risk_level::RiskLevel; 
use crate::disclaimer::Disclaimer; 
use crate::turnover::Turnover; 

// It's a struct
/// Jurisdiction Risk Assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct JurisdictionRiskAssessment {
    /// Disclaimer configuration\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub disclaimer: Option<Disclaimer>,
    /// Risk level configuration\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub risk_level: Option<RiskLevel>,
    /// Turnover configuration\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub turnover: Option<Turnover>,
}

