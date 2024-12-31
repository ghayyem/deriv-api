
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/residence_list/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// A message with Residence List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct ResidenceListResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// List of countries for account opening
    #[serde(rename = "residence_list", skip_serializing_if = "Option::is_none")]
    pub residence_list: Vec<ResidenceListitem>,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct ResidenceListitem {
    /// Flag which indicates whether self declaration is required for account opening
    #[serde(rename = "account_opening_self_declaration_required", skip_serializing_if = "Option::is_none")]
    pub account_opening_self_declaration_required: AccountOpeningSelfDeclarationRequiredEnum,
    /// Common Reporting Standard
    #[serde(rename = "common_reporting_standard", skip_serializing_if = "Option::is_none")]
    pub common_reporting_standard: CommonReportingStandard,
    /// Disabled.
    #[serde(rename = "disabled", skip_serializing_if = "Option::is_none")]
    pub disabled: String,
    /// Information about identity options available
    #[serde(rename = "identity", skip_serializing_if = "Option::is_none")]
    pub identity: Identity,
    /// Jurisdiction Risk Assessment
    #[serde(rename = "jurisdiction_risk_assessment", skip_serializing_if = "Option::is_none")]
    pub jurisdiction_risk_assessment: JurisdictionRiskAssessment,
    /// Flag which indicates whether partner signup is available in this country
    #[serde(rename = "partner_signup", skip_serializing_if = "Option::is_none")]
    pub partner_signup: PartnerSignupEnum,
    /// IDD code of country
    #[serde(rename = "phone_idd", skip_serializing_if = "Option::is_none")]
    pub phone_idd: Option<Value>,
    /// Selected.
    #[serde(rename = "selected", skip_serializing_if = "Option::is_none")]
    pub selected: String,
    /// Country full name
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: String,
    /// Country tax identifier format
    #[serde(rename = "tin_format", skip_serializing_if = "Option::is_none")]
    pub tin_format: Vec<String>,
    /// 2-letter country code
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: String,
    /// Flag which indicates whether wallet signup is available in this country
    #[serde(rename = "wallet_signup", skip_serializing_if = "Option::is_none")]
    pub wallet_signup: WalletSignupEnum,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Common Reporting Standard
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct CommonReportingStandard {
    /// NPJ configuration
    #[serde(rename = "non_participating_jurisdictions", skip_serializing_if = "Option::is_none")]
    pub non_participating_jurisdictions: NonParticipatingJurisdictions,
    /// Postcode configuration
    #[serde(rename = "postcode", skip_serializing_if = "Option::is_none")]
    pub postcode: Postcode,
    /// Tax configuration
    #[serde(rename = "tax", skip_serializing_if = "Option::is_none")]
    pub tax: Tax,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// NPJ configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct NonParticipatingJurisdictions {
    /// Default NPJ flag
    #[serde(rename = "default", skip_serializing_if = "Option::is_none")]
    pub default: bool,
    /// Flags for specific landing companies
    #[serde(rename = "landing_company", skip_serializing_if = "Option::is_none", flatten)]
    pub landing_company: HashMap<String, bool>,
}






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Postcode configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Postcode {
    /// Invalid regex patterns for postcode validation
    #[serde(rename = "invalid_pattern", skip_serializing_if = "Option::is_none")]
    pub invalid_pattern: Option<Value>,
}






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Tax configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Tax {
    /// Mandatory TIN flag
    #[serde(rename = "mandatory", skip_serializing_if = "Option::is_none")]
    pub mandatory: bool,
    /// POI equivalent flag
    #[serde(rename = "poi_equivalent", skip_serializing_if = "Option::is_none")]
    pub poi_equivalent: bool,
    /// Cleanup regex
    #[serde(rename = "tin_cleaner", skip_serializing_if = "Option::is_none")]
    pub tin_cleaner: String,
    /// Country tax identifier format
    #[serde(rename = "tin_format", skip_serializing_if = "Option::is_none")]
    pub tin_format: Vec<String>,
    /// Description of the TIN format
    #[serde(rename = "tin_format_description", skip_serializing_if = "Option::is_none")]
    pub tin_format_description: Option<Value>,
}








// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Jurisdiction Risk Assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct JurisdictionRiskAssessment {
    /// Disclaimer configuration
    #[serde(rename = "disclaimer", skip_serializing_if = "Option::is_none")]
    pub disclaimer: Disclaimer,
    /// Risk level configuration
    #[serde(rename = "risk_level", skip_serializing_if = "Option::is_none")]
    pub risk_level: RiskLevel,
    /// Turnover configuration
    #[serde(rename = "turnover", skip_serializing_if = "Option::is_none")]
    pub turnover: Turnover,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Disclaimer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Disclaimer {
    /// Disclaimer flag
    #[serde(rename = "accept", skip_serializing_if = "Option::is_none")]
    pub accept: bool,
    /// Disclaimer message
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: String,
}






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Risk level configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct RiskLevel {
    /// Default risk level flag
    #[serde(rename = "default", skip_serializing_if = "Option::is_none")]
    pub default: String,
    /// Flags for specific landing companies
    #[serde(rename = "landing_company", skip_serializing_if = "Option::is_none", flatten)]
    pub landing_company: HashMap<String, String>,
}






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Turnover configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Turnover {
    /// Max limit
    #[serde(rename = "max_limit", skip_serializing_if = "Option::is_none")]
    pub max_limit: bool,
}








/// Flag which indicates whether self declaration is required for account opening
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountOpeningSelfDeclarationRequiredEnum {
    Value0,
    Value1 = 1,
}

impl AccountOpeningSelfDeclarationRequiredEnum {
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
/// Flag which indicates whether partner signup is available in this country
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PartnerSignupEnum {
    Value0,
    Value1 = 1,
}

impl PartnerSignupEnum {
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
/// Flag which indicates whether wallet signup is available in this country
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WalletSignupEnum {
    Value1 = 1,
}

impl WalletSignupEnum {
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


