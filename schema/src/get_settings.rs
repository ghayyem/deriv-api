
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/get_settings/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 
use serde_json::Value;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

// Import shared types from the *same* crate
use crate::email_consent_enum::EmailConsentEnum; 
use crate::request_professional_status_enum::RequestProfessionalStatusEnum; 
use crate::has_secret_answer_enum::HasSecretAnswerEnum; 
use crate::tin_skipped_enum::TinSkippedEnum; 
use crate::is_authenticated_payment_agent_enum::IsAuthenticatedPaymentAgentEnum; 
use crate::dxtrade_user_exception_enum::DxtradeUserExceptionEnum; 
use crate::allow_copiers_enum::AllowCopiersEnum; 
use crate::phone_number_verification::PhoneNumberVerification; 
use crate::feature_flag::FeatureFlag; 
use crate::non_pep_declaration_enum::NonPepDeclarationEnum; 

// It's a struct
/// User information and settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetSettings {
    /// Field 'account_opening_reason' mapped to Value due to complexity/potential issues.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub account_opening_reason: Option<Value>,
    /// City (note: Only available for users who have at least one real account)\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub address_city: Option<String>,
    /// Address line 1 (note: Only available for users who have at least one real account)\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub address_line_1: Option<String>,
    /// Field 'address_line_2' mapped to Value due to complexity/potential issues.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub address_line_2: Option<Value>,
    /// Post Code (note: Only available for users who have at least one real account)\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub address_postcode: Option<String>,
    /// State (note: Only available for users who have at least one real account)\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub address_state: Option<String>,
    /// Boolean value 1 or 0, indicating permission to allow others to follow your trades. Note: not applicable for Virtual account. Only allow for real money account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub allow_copiers: Option<AllowCopiersEnum>,
    /// The phone's calling country code.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub calling_country_code: Option<String>,
    /// Country of legal citizenship, 2-letter country code.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub citizen: Option<String>,
    /// Latest terms and conditions version accepted by client\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub client_tnc_status: Option<String>,
    /// Cooldown expiration epoch date when a client fails appropriateness tests\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub cooling_off_expiration_date: Option<DateTime<Utc>>,
    /// User Country (same as residence field) - deprecated\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub country: Option<String>,
    /// 2-letter country code ISO standard\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub country_code: Option<String>,
    /// Epoch of user's birthday (note: Only available for users who have at least one real account)\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub date_of_birth: Option<i64>,
    /// Boolean value 1 or 0, indicating if user email belong to dxtrade exception list.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub dxtrade_user_exception: Option<DxtradeUserExceptionEnum>,
    /// User Email\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub email: Option<String>,
    /// Boolean value 1 or 0, indicating permission to use email address for any contact which may include marketing\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub email_consent: Option<EmailConsentEnum>,
    /// Employment Status.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub employment_status: Option<String>,
    /// Indicates client's self-declaration for FATCA.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub fatca_declaration: Option<i64>,
    /// Contains features that are enabled or disabled for this user\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub feature_flag: Option<FeatureFlag>,
    /// First name (note: Only available for users who have at least one real account)\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub first_name: Option<String>,
    /// Returns 1 if the client has a secret answer, 0 otherwise.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub has_secret_answer: Option<HasSecretAnswerEnum>,
    /// A list of profile fields which are immutable (read-only unless they are not set yet) due to landing company regulations and the current status of the account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub immutable_fields: Option<Vec<String>>,
    /// Boolean value 1 or 0, indicating whether is payment agent (note: not applicable for virtual money accounts)\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub is_authenticated_payment_agent: Option<IsAuthenticatedPaymentAgentEnum>,
    /// Last name (note: Only available for users who have at least one real account)\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub last_name: Option<String>,
    /// Indicates client's self-declaration of not being a PEP/RCA (Politically Exposed Person/Relatives and Close Associates). Note: returned for real accounts only.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub non_pep_declaration: Option<NonPepDeclarationEnum>,
    /// The phone's national format phone.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub phone: Option<String>,
    /// The status of the Phone Number Verification.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub phone_number_verification: Option<PhoneNumberVerification>,
    /// Place of birth, 2-letter country code.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub place_of_birth: Option<String>,
    /// User's preferred language, ISO standard code of language\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub preferred_language: Option<String>,
    /// Boolean value 1 or 0, indicating if client has requested professional status.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub request_professional_status: Option<RequestProfessionalStatusEnum>,
    /// User Country\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub residence: Option<String>,
    /// Salutation (note: Only available for users who have at least one real account)\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub salutation: Option<String>,
    /// Tax identification number. Only applicable for real money account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub tax_identification_number: Option<String>,
    /// Residence for tax purpose. Comma separated iso country code if multiple jurisdictions. Only applicable for real money account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub tax_residence: Option<String>,
    /// [Optional] Whether the client has skipped the TIN form. Only applicable for real money account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub tin_skipped: Option<TinSkippedEnum>,
    /// Terms and condition status tells us whether all the accounts of this user has accepted the latest T&C version.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub tnc_status: Option<Value>,
    /// The start date to inform users within a grace period, with the notification automatically deactivating after the specified duration.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub tnc_update_notification_start_date: Option<String>,
    /// Boolean value 1 or 0, indicating if client has enabled the Trading Hub dashboard\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub trading_hub: Option<i64>,
    /// Hash generated using user details to verify whether the user is legitimate for our customer support system.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub user_hash: Option<String>,
}

