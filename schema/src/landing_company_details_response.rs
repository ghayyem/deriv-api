
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




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// The configuration of each currency.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct CurrencyConfig {
    /// Name of commodities.
    #[serde(rename = "commodities", skip_serializing_if = "Option::is_none")]
    pub commodities: Commodities,
    /// Name of cryptocurrency.
    #[serde(rename = "cryptocurrency", skip_serializing_if = "Option::is_none")]
    pub cryptocurrency: Cryptocurrency,
    /// Name of forex.
    #[serde(rename = "forex", skip_serializing_if = "Option::is_none")]
    pub forex: Forex,
    /// Name of indices.
    #[serde(rename = "indices", skip_serializing_if = "Option::is_none")]
    pub indices: Indices,
    /// Name of market.
    #[serde(rename = "market", skip_serializing_if = "Option::is_none")]
    pub market: Market,
    /// Name of synthetic index.
    #[serde(rename = "synthetic_index", skip_serializing_if = "Option::is_none")]
    pub synthetic_index: SyntheticIndex,
}






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Legal requirements for the given Landing Company.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Requirements {
    /// After first deposit requirements
    #[serde(rename = "after_first_deposit", skip_serializing_if = "Option::is_none")]
    pub after_first_deposit: AfterFirstDeposit,
    /// Compliance requirements
    #[serde(rename = "compliance", skip_serializing_if = "Option::is_none")]
    pub compliance: Compliance,
    /// Sign up requirements
    #[serde(rename = "signup", skip_serializing_if = "Option::is_none")]
    pub signup: Vec<String>,
    /// Withdrawal requirements
    #[serde(rename = "withdrawal", skip_serializing_if = "Option::is_none")]
    pub withdrawal: Vec<String>,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// After first deposit requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct AfterFirstDeposit {
    /// Financial assessment requirements
    #[serde(rename = "financial_assessment", skip_serializing_if = "Option::is_none")]
    pub financial_assessment: Vec<String>,
}






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Compliance requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Compliance {
    /// Compliance MT5 requirements
    #[serde(rename = "mt5", skip_serializing_if = "Option::is_none")]
    pub mt_5: Vec<String>,
    /// Compliance tax information requirements
    #[serde(rename = "tax_information", skip_serializing_if = "Option::is_none")]
    pub tax_information: Vec<String>,
}








/// Flag to indicate whether reality check is applicable for this Landing Company. `1`: applicable, `0`: not applicable. The Reality Check is a feature that gives a summary of the client's trades and account balances on a regular basis throughout his session, and is a regulatory requirement for certain Landing Companies.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HasRealityCheckEnum {
    Value0,
    Value1 = 1,
}

impl HasRealityCheckEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Value0 => "0",
            Self::Value1 => "1",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "0" => Some(Self::Value0),
            "1" => Some(Self::Value1),
            _ => None,
        }
    }
}
/// Flag that indicates whether the landing company supports professional accounts or not
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportProfessionalClientEnum {
    Value0,
    Value1 = 1,
}

impl SupportProfessionalClientEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Value0 => "0",
            Self::Value1 => "1",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "0" => Some(Self::Value0),
            "1" => Some(Self::Value1),
            _ => None,
        }
    }
}
/// Flag that indicates whether tax identifier number is not mandatory for the current country and landing company.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TinNotMandatoryEnum {
    Value0,
    Value1 = 1,
}

impl TinNotMandatoryEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Value0 => "0",
            Self::Value1 => "1",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "0" => Some(Self::Value0),
            "1" => Some(Self::Value1),
            _ => None,
        }
    }
}


