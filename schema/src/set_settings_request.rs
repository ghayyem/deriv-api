
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/set_settings/send.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


// Import required types from the *same* crate
use crate::salutation_enum::SalutationEnum;
use crate::secret_question_enum::SecretQuestionEnum;
use crate::set_settings_enum::SetSettingsEnum;
use crate::email_consent_enum::EmailConsentEnum;
use crate::request_professional_status_enum::RequestProfessionalStatusEnum;
use crate::trading_hub_enum::TradingHubEnum;
use crate::tin_skipped_enum::TinSkippedEnum;
use crate::dxtrade_user_exception_enum::DxtradeUserExceptionEnum;
use crate::allow_copiers_enum::AllowCopiersEnum;
use crate::feature_flag::FeatureFlag;
use crate::non_pep_declaration_enum::NonPepDeclarationEnum;

/// Set User Settings (this call should be used in conjunction with `get_settings`)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetSettingsRequest {
    /// Field 'account_opening_reason' mapped to Value due to complexity/potential issues.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub account_opening_reason: Option<Value>,
    /// [Optional] Note: not applicable for virtual account. Required field for real money account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub address_city: Option<String>,
    /// [Optional] Note: not applicable for virtual account. Required field for real money account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub address_line_1: Option<String>,
    /// Field 'address_line_2' mapped to Value due to complexity/potential issues.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub address_line_2: Option<Value>,
    /// [Optional] Note: not applicable for virtual account. Optional field for real money account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub address_postcode: Option<String>,
    /// [Optional] Note: not applicable for virtual account. Optional field for real money account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub address_state: Option<String>,
    /// [Optional] Boolean value 1 or 0, indicating permission to allow others to follow your trades. Note: not applicable for Virtual account. Only allow for real money account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub allow_copiers: Option<AllowCopiersEnum>,
    /// [Optional] The phone's calling country code. Don't include the `+` sign. Up to 4 digits.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub calling_country_code: Option<String>,
    /// [Optional] Country of legal citizenship, 2-letter country code.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub citizen: Option<String>,
    /// [Optional] Date of birth format: yyyy-mm-dd (can only be changed on unauthenticated svg accounts).\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub date_of_birth: Option<String>,
    /// Boolean value 1 or 0, indicating if user email belong to dxtrade exception list.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub dxtrade_user_exception: Option<DxtradeUserExceptionEnum>,
    /// [Optional] Boolean value 1 or 0, indicating permission to use email address for any contact which may include marketing\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub email_consent: Option<EmailConsentEnum>,
    /// [Optional] Employment Status.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub employment_status: Option<String>,
    /// [Optional] Enable or disable one or multiple features.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub feature_flag: Option<FeatureFlag>,
    /// [Optional] The version of the financial information\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub financial_information_version: Option<String>,
    /// [Optional] Within 1-50 characters, use only letters, spaces, hyphens, full-stops or apostrophes (can only be changed on unauthenticated svg accounts).\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub first_name: Option<String>,
    /// [Optional] Within 1-50 characters, use only letters, spaces, hyphens, full-stops or apostrophes (can only be changed on unauthenticated svg accounts).\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub last_name: Option<String>,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub loginid: Option<String>,
    /// [Optional] Indicates client's self-declaration of not being a PEP/RCA (Politically Exposed Person/Relatives and Close Associates). Effective for real accounts only.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub non_pep_declaration: Option<NonPepDeclarationEnum>,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub passthrough: Option<Value>,
    /// [Optional] The phone's national format, don't include the `+` sign nor the calling country code. Up to 15 digits are allowed.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub phone: Option<String>,
    /// [Optional] Place of birth, 2-letter country code.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub place_of_birth: Option<String>,
    /// [Optional] User's preferred language, ISO standard language code\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub preferred_language: Option<String>,
    /// [Optional] Used to map request to response.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub req_id: Option<i64>,
    /// [Optional] Required when client wants to be treated as professional. Applicable for financial accounts only.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub request_professional_status: Option<RequestProfessionalStatusEnum>,
    /// [Optional] 2-letter country code. Note: not applicable for real money account. Only allow for Virtual account without residence set.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub residence: Option<String>,
    /// [Optional] Accept any value in enum list (can only be changed on unauthenticated svg accounts).\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub salutation: Option<SalutationEnum>,
    /// [Optional] Answer to secret question, within 4-50 characters. Required for new account and existing client details will be used if client opens another account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub secret_answer: Option<String>,
    /// [Optional] Accept any value in enum list. Required for new account and existing client details will be used if client opens another account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub secret_question: Option<SecretQuestionEnum>,
    /// Must be `1`\n
    // Correct serde attribute construction - Use helper
    
    pub set_settings: SetSettingsEnum,
    /// [Optional] Tax identification number. Only applicable for real money account. Required for maltainvest landing company.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub tax_identification_number: Option<String>,
    /// [Optional] Residence for tax purpose. Comma separated iso country code if multiple jurisdictions. Only applicable for real money account. Required for maltainvest landing company.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub tax_residence: Option<String>,
    /// [Optional] Whether the client has skipped the TIN form. Only applicable for real money account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub tin_skipped: Option<TinSkippedEnum>,
    /// [Optional] Enable/Disable Trading Hub dashboard\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub trading_hub: Option<TradingHubEnum>,
}

