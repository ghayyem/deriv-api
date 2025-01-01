
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/new_partner_account/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// This call opens a new Real-Partner Account
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct NewPartnerAccountRequest {
    /// [Optional] The anticipated account turnover.
    #[serde(rename = "account_turnover", skip_serializing_if = "Option::is_none")]
    pub account_turnover: AccountTurnoverEnum,
    /// [Optional] Within 100 characters.
    #[serde(rename = "address_city", skip_serializing_if = "Option::is_none")]
    pub address_city: String,
    /// Within 70 characters, with no leading whitespaces and may contain letters/numbers and/or any of following characters '.,:;()@#/-
    #[serde(rename = "address_line_1", skip_serializing_if = "Option::is_none")]
    pub address_line_1: String,
    /// [Optional] Within 70 characters.
    #[serde(rename = "address_line_2", skip_serializing_if = "Option::is_none")]
    pub address_line_2: String,
    /// [Optional] Within 20 characters and may not contain '+'.
    #[serde(rename = "address_postcode", skip_serializing_if = "Option::is_none")]
    pub address_postcode: String,
    /// [Optional] Possible value receive from `states_list` call.
    #[serde(rename = "address_state", skip_serializing_if = "Option::is_none")]
    pub address_state: String,
    /// [Optional] The phone's calling country code.
    #[serde(rename = "calling_country_code", skip_serializing_if = "Option::is_none")]
    pub calling_country_code: Option<Value>,
    /// [Optional] Country of legal citizenship, 2-letter country code.
    #[serde(rename = "citizen", skip_serializing_if = "Option::is_none")]
    pub citizen: Option<Value>,
    /// [Optional] Indicates whether this is for a client requesting an account with professional status.
    #[serde(rename = "client_type", skip_serializing_if = "Option::is_none")]
    pub client_type: ClientTypeEnum,
    /// [Optional] Company name. Only applicable for partners of type company.
    #[serde(rename = "company_name", skip_serializing_if = "Option::is_none")]
    pub company_name: String,
    /// [Optional] Company registration number. Only applicable for partners of type company.
    #[serde(rename = "company_registration_no", skip_serializing_if = "Option::is_none")]
    pub company_registration_no: String,
    /// [Optional] To set currency of the account. List of supported currencies can be acquired with `payout_currencies` call.
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: String,
    /// Date of birth format: `yyyy-mm-dd`.
    #[serde(rename = "date_of_birth", skip_serializing_if = "Option::is_none")]
    pub date_of_birth: String,
    /// [Optional] Indicates client's self-declaration of FATCA.
    #[serde(rename = "fatca_declaration", skip_serializing_if = "Option::is_none")]
    pub fatca_declaration: FatcaDeclarationEnum,
    /// Within 1-50 characters, use only letters, spaces, hyphens, full-stops or apostrophes.
    #[serde(rename = "first_name", skip_serializing_if = "Option::is_none")]
    pub first_name: String,
    /// [Optional] If specified, will return only the underlyings for the specified landing company.
    #[serde(rename = "landing_company_short", skip_serializing_if = "Option::is_none")]
    pub landing_company_short: LandingCompanyShortEnum,
    /// Within 1-50 characters, use only letters, spaces, hyphens, full-stops or apostrophes.
    #[serde(rename = "last_name", skip_serializing_if = "Option::is_none")]
    pub last_name: String,
    /// [Optional] The login id of the user. If left unspecified, it defaults to the initial authorized token's login id.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// Must be `1`
    #[serde(rename = "new_partner_account")]
    pub new_partner_account: NewPartnerAccountEnum,
    /// [Optional] Indicates client's self-declaration of not being a PEP/RCA (Politically Exposed Person/Relatives and Close Associates).
    #[serde(rename = "non_pep_declaration", skip_serializing_if = "Option::is_none")]
    pub non_pep_declaration: i64,
    /// Defines whether this partner is an individual or a company. Only applicable for partners
    #[serde(rename = "partner_type")]
    pub partner_type: PartnerTypeEnum,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] Starting with `+` followed by 8-35 digits, allowing hyphens or space.
    #[serde(rename = "phone", skip_serializing_if = "Option::is_none")]
    pub phone: String,
    /// [Optional] Name  of the provider platform.
    #[serde(rename = "provider", skip_serializing_if = "Option::is_none")]
    pub provider: ProviderEnum,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// 2-letter country code, possible value receive from `residence_list` call.
    #[serde(rename = "residence", skip_serializing_if = "Option::is_none")]
    pub residence: String,
    /// [Optional] Accept any value in enum list.
    #[serde(rename = "salutation", skip_serializing_if = "Option::is_none")]
    pub salutation: SalutationEnum,
    /// [Optional] Answer to secret question, within 4-50 characters. Required for new account and existing client details will be used if client open another account.
    #[serde(rename = "secret_answer", skip_serializing_if = "Option::is_none")]
    pub secret_answer: String,
    /// [Optional] Accept any value in enum list. Required for new account and existing client details will be used if client open another account.
    #[serde(rename = "secret_question", skip_serializing_if = "Option::is_none")]
    pub secret_question: SecretQuestionEnum,
    /// Partner's Website URI/Promotional Platform
    #[serde(rename = "website")]
    pub website: String,
}




/// Must be `1`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NewPartnerAccountEnum {
    Value1 = 1,
}

impl NewPartnerAccountEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Value1 => "1",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "1" => Some(Self::Value1),
            _ => None,
        }
    }
}
/// Defines whether this partner is an individual or a company. Only applicable for partners
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PartnerTypeEnum {
    Individual,
    Company,
}

impl PartnerTypeEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Individual => "individual",
            Self::Company => "company",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "individual" => Some(Self::Individual),
            "company" => Some(Self::Company),
            _ => None,
        }
    }
}
