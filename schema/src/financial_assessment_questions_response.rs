
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/financial_assessment_questions/receive.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;



// Import required types from the *same* crate
use crate::financial_assessment_questions::FinancialAssessmentQuestions;

/// Returns the questionnaire structure defining the financial assessment questions and logic
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FinancialAssessmentQuestionsResponse {
    /// Echo of the request made.\n
    // Correct serde attribute construction - Use helper
    
    pub echo_req: Value,
    /// The financial assessment questionnaire structure\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub financial_assessment_questions: Option<FinancialAssessmentQuestions>,
    /// Action name of the request made.\n
    // Correct serde attribute construction - Use helper
    
    pub msg_type: String,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub req_id: Option<i64>,
}

