
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/residence_list/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 
use serde_json::Value;



// Import shared types from the *same* crate
use crate::jurisdiction_risk_assessment::JurisdictionRiskAssessment; 
use crate::identity::Identity; 
use crate::partner_signup::PartnerSignup; 
use crate::common_reporting_standard::CommonReportingStandard; 
use crate::account_opening_self_declaration_required::AccountOpeningSelfDeclarationRequired; 

// It's a struct
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ResidenceListItem {
    /// Flag which indicates whether self declaration is required for account opening\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub account_opening_self_declaration_required: Option<AccountOpeningSelfDeclarationRequired>,
    /// Common Reporting Standard\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub common_reporting_standard: Option<CommonReportingStandard>,
    /// Disabled.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub disabled: Option<String>,
    /// Information about identity options available\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub identity: Option<Identity>,
    /// Jurisdiction Risk Assessment\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub jurisdiction_risk_assessment: Option<JurisdictionRiskAssessment>,
    /// Flag which indicates whether partner signup is available in this country\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub partner_signup: Option<PartnerSignup>,
    /// IDD code of country\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub phone_idd: Option<Value>,
    /// Selected.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub selected: Option<String>,
    /// Country full name\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub text: Option<String>,
    /// Country tax identifier format\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub tin_format: Option<Vec<String>>,
    /// 2-letter country code\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub value: Option<String>,
    /// Flag which indicates whether wallet signup is available in this country\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub wallet_signup: Option<i64>,
}

