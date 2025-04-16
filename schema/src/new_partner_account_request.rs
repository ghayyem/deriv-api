
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/new_partner_account/send.json

// Use direct crate names for imports
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


// Import required types from the *same* crate
use crate::account_turnover_enum::AccountTurnoverEnum;
use crate::salutation_enum::SalutationEnum;
use crate::provider_enum::ProviderEnum;
use crate::secret_question_enum::SecretQuestionEnum;
use crate::client_type_enum::ClientTypeEnum;
use crate::partner_type_enum::PartnerTypeEnum;
use crate::new_partner_account_enum::NewPartnerAccountEnum;
use crate::landing_company_short_enum::LandingCompanyShortEnum;
use crate::fatca_declaration_enum::FatcaDeclarationEnum;

/// This call opens a new Real-Partner Account
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NewPartnerAccountRequest {
    /// [Optional] The anticipated account turnover.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub account_turnover: Option<AccountTurnoverEnum>,
    /// [Optional] Within 100 characters.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub address_city: Option<String>,
    /// Within 70 characters, with no leading whitespaces and may contain letters/numbers and/or any of following characters '.,:;()@#/-\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub address_line_1: Option<String>,
    /// Field 'address_line_2' mapped to Value due to complexity/potential issues.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub address_line_2: Option<Value>,
    /// [Optional] Within 20 characters and may not contain '+'.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub address_postcode: Option<String>,
    /// [Optional] Possible value receive from `states_list` call.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub address_state: Option<f64>,
    /// [Optional] The phone's calling country code.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub calling_country_code: Option<String>,
    /// [Optional] Country of legal citizenship, 2-letter country code.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub citizen: Option<String>,
    /// [Optional] Indicates whether this is for a client requesting an account with professional status.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub client_type: Option<ClientTypeEnum>,
    /// [Optional] Company name. Only applicable for partners of type company.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub company_name: Option<String>,
    /// [Optional] Company registration number. Only applicable for partners of type company.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub company_registration_no: Option<String>,
    /// [Optional] To set currency of the account. List of supported currencies can be acquired with `payout_currencies` call.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub currency: Option<String>,
    /// Date of birth format: `yyyy-mm-dd`.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub date_of_birth: Option<String>,
    /// [Optional] Indicates client's self-declaration of FATCA.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub fatca_declaration: Option<FatcaDeclarationEnum>,
    /// Within 1-50 characters, use only letters, spaces, hyphens, full-stops or apostrophes.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub first_name: Option<String>,
    /// [Optional] If specified, will return only the underlyings for the specified landing company.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub landing_company_short: Option<LandingCompanyShortEnum>,
    /// Within 1-50 characters, use only letters, spaces, hyphens, full-stops or apostrophes.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub last_name: Option<String>,
    /// [Optional] The login id of the user. If left unspecified, it defaults to the initial authorized token's login id.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub loginid: Option<String>,
    /// Must be `1`\n
    // Correct serde attribute construction - Use helper
    
    pub new_partner_account: NewPartnerAccountEnum,
    /// [Optional] Indicates client's self-declaration of not being a PEP/RCA (Politically Exposed Person/Relatives and Close Associates).\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub non_pep_declaration: Option<i64>,
    /// Defines whether this partner is an individual or a company. Only applicable for partners\n
    // Correct serde attribute construction - Use helper
    
    pub partner_type: PartnerTypeEnum,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub passthrough: Option<Value>,
    /// [Optional] Starting with `+` followed by 8-35 digits, allowing hyphens or space.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub phone: Option<String>,
    /// [Optional] Name  of the provider platform.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub provider: Option<ProviderEnum>,
    /// [Optional] Used to map request to response.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub req_id: Option<i64>,
    /// 2-letter country code, possible value receive from `residence_list` call.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub residence: Option<f64>,
    /// [Optional] Accept any value in enum list.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub salutation: Option<SalutationEnum>,
    /// [Optional] Answer to secret question, within 4-50 characters. Required for new account and existing client details will be used if client open another account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub secret_answer: Option<String>,
    /// [Optional] Accept any value in enum list. Required for new account and existing client details will be used if client open another account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub secret_question: Option<SecretQuestionEnum>,
    /// Partner's Website URI/Promotional Platform\n
    // Correct serde attribute construction - Use helper
    
    pub website: String,
}

