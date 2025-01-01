
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/landing_company_details/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// A message with Landing Company.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct LandingCompanyDetailsResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// The detailed information of the requested landing company.
    #[serde(rename = "landing_company_details", skip_serializing_if = "Option::is_none")]
    pub landing_company_details: LandingCompanyDetails,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// The detailed information of the requested landing company.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct LandingCompanyDetails {
    /// Landing Company address.
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<Value>,
    /// Special conditions for changing sensitive fields
    #[serde(rename = "changeable_fields", skip_serializing_if = "Option::is_none")]
    pub changeable_fields: ChangeableFields,
    /// Landing Company country.
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: String,
    /// The configuration of each currency.
    #[serde(rename = "currency_config", skip_serializing_if = "Option::is_none")]
    pub currency_config: CurrencyConfig,
    /// Flag to indicate whether reality check is applicable for this Landing Company. `1`: applicable, `0`: not applicable. The Reality Check is a feature that gives a summary of the client's trades and account balances on a regular basis throughout his session, and is a regulatory requirement for certain Landing Companies.
    #[serde(rename = "has_reality_check", skip_serializing_if = "Option::is_none")]
    pub has_reality_check: HasRealityCheckEnum,
    /// Allowed contract types for this Landing Company
    #[serde(rename = "legal_allowed_contract_categories", skip_serializing_if = "Option::is_none")]
    pub legal_allowed_contract_categories: Vec<String>,
    /// Allowable currencies for accounts with this Landing Company.
    #[serde(rename = "legal_allowed_currencies", skip_serializing_if = "Option::is_none")]
    pub legal_allowed_currencies: Vec<String>,
    /// Allowed markets for this Landing Company
    #[serde(rename = "legal_allowed_markets", skip_serializing_if = "Option::is_none")]
    pub legal_allowed_markets: Vec<String>,
    /// Default currency of client accounts with this Landing Company.
    #[serde(rename = "legal_default_currency", skip_serializing_if = "Option::is_none")]
    pub legal_default_currency: String,
    /// Landing Company name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Legal requirements for the given Landing Company.
    #[serde(rename = "requirements", skip_serializing_if = "Option::is_none")]
    pub requirements: Requirements,
    /// Landing Company shortcode.
    #[serde(rename = "shortcode", skip_serializing_if = "Option::is_none")]
    pub shortcode: String,
    /// Flag that indicates whether the landing company supports professional accounts or not
    #[serde(rename = "support_professional_client", skip_serializing_if = "Option::is_none")]
    pub support_professional_client: SupportProfessionalClientEnum,
    /// Flag that indicates whether tax identifier number is not mandatory for the current country and landing company.
    #[serde(rename = "tin_not_mandatory", skip_serializing_if = "Option::is_none")]
    pub tin_not_mandatory: TinNotMandatoryEnum,
}






