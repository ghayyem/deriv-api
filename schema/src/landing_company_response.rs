
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/landing_company/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Returns the Landing Company for clients of a given country.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct LandingCompanyResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Landing Company
    #[serde(rename = "landing_company", skip_serializing_if = "Option::is_none")]
    pub landing_company: LandingCompany,
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


/// Landing Company
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct LandingCompany {
    /// Flag to indicate if address parseable or not
    #[serde(rename = "address_parseable", skip_serializing_if = "Option::is_none")]
    pub address_parseable: AddressParseableEnum,
    /// Config for all account types (Synthetic Indices and Financials).
    #[serde(rename = "all_company", skip_serializing_if = "Option::is_none")]
    pub all_company: AllCompanyEnum,
    /// Config structure with document types ,taxRequired ,tin format details.
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Config,
    /// Available CTrader accounts.
    #[serde(rename = "ctrader", skip_serializing_if = "Option::is_none")]
    pub ctrader: Ctrader,
    /// Available Deriv X all account types (Synthetic Indices and Financials).
    #[serde(rename = "dxtrade_all_company", skip_serializing_if = "Option::is_none")]
    pub dxtrade_all_company: DxtradeAllCompany,
    /// Available Deriv X financial account types (all except Synthetic Indices).
    #[serde(rename = "dxtrade_financial_company", skip_serializing_if = "Option::is_none")]
    pub dxtrade_financial_company: DxtradeFinancialCompany,
    /// Available Deriv X derived account types (Synthetic Indices).
    #[serde(rename = "dxtrade_gaming_company", skip_serializing_if = "Option::is_none")]
    pub dxtrade_gaming_company: DxtradeGamingCompany,
    /// Landing Company for financial contracts (all except Synthetic Indices)
    #[serde(rename = "financial_company", skip_serializing_if = "Option::is_none")]
    pub financial_company: Option<Value>,
    /// Forbidden postcode pattern
    #[serde(rename = "forbidden_postcode_pattern", skip_serializing_if = "Option::is_none")]
    pub forbidden_postcode_pattern: String,
    /// Landing Company for derived contracts (Synthetic Indices)
    #[serde(rename = "gaming_company", skip_serializing_if = "Option::is_none")]
    pub gaming_company: Option<Value>,
    /// Country code
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    /// Flag to indicate if idv is supported or not
    #[serde(rename = "is_idv_supported", skip_serializing_if = "Option::is_none")]
    pub is_idv_supported: IsIdvSupportedEnum,
    /// Open mf account lc details.
    #[serde(rename = "lc_to_open_mf_account", skip_serializing_if = "Option::is_none")]
    pub lc_to_open_mf_account: String,
    /// Minimum age
    #[serde(rename = "minimum_age", skip_serializing_if = "Option::is_none")]
    pub minimum_age: i64,
    /// Flag to indicate if mt5 age verification detail.
    #[serde(rename = "mt5_age_verification", skip_serializing_if = "Option::is_none")]
    pub mt_5_age_verification: Mt5AgeVerificationEnum,
    /// Landing Company for MT5 standard combined all Synthetic and financial, currently has Financial as subtype.
    #[serde(rename = "mt_all_company", skip_serializing_if = "Option::is_none")]
    pub mt_all_company: Option<Value>,
    /// Landing Company for MT5 financial contracts (all except Synthetic Indices), currently divided into Financial STP, Financial (standard) as subtypes.
    #[serde(rename = "mt_financial_company", skip_serializing_if = "Option::is_none")]
    pub mt_financial_company: Option<Value>,
    /// Landing Company for MT5 standard derived contracts (Synthetic Indices), currently has Financial as subtype.
    #[serde(rename = "mt_gaming_company", skip_serializing_if = "Option::is_none")]
    pub mt_gaming_company: Option<Value>,
    /// Country name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Flag to indicate whether max turnover limit settings.
    #[serde(rename = "need_set_max_turnover_limit", skip_serializing_if = "Option::is_none")]
    pub need_set_max_turnover_limit: NeedSetMaxTurnoverLimitEnum,
    /// Flag to indicate province settings.
    #[serde(rename = "no_province", skip_serializing_if = "Option::is_none")]
    pub no_province: NoProvinceEnum,
    /// Flag to indicate whether address postcode is required or not.
    #[serde(rename = "require_address_postcode", skip_serializing_if = "Option::is_none")]
    pub require_address_postcode: RequireAddressPostcodeEnum,
    /// Flag to indicate whether age verification required ofr synthetic or not.
    #[serde(rename = "require_age_verified_for_synthetic", skip_serializing_if = "Option::is_none")]
    pub require_age_verified_for_synthetic: RequireAgeVerifiedForSyntheticEnum,
    /// Flag to indicate whether poi is required.
    #[serde(rename = "require_poi", skip_serializing_if = "Option::is_none")]
    pub require_poi: RequirePoiEnum,
    /// Flag to indicate whether verification required if age not verified.
    #[serde(rename = "require_verification_when_not_age_verified", skip_serializing_if = "Option::is_none")]
    pub require_verification_when_not_age_verified: RequireVerificationWhenNotAgeVerifiedEnum,
    /// Flag to indicate whether to skip deposit verifcation or not.
    #[serde(rename = "skip_deposit_verification", skip_serializing_if = "Option::is_none")]
    pub skip_deposit_verification: SkipDepositVerificationEnum,
    /// Virtual Company
    #[serde(rename = "virtual_company", skip_serializing_if = "Option::is_none")]
    pub virtual_company: String,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Available CTrader accounts.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Ctrader {
    /// CTrader all account types (Synthetic Indices and Financials).
    #[serde(rename = "all", skip_serializing_if = "Option::is_none")]
    pub all: All,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// CTrader all account types (Synthetic Indices and Financials).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct All {
    /// For standard client
    #[serde(rename = "standard", skip_serializing_if = "Option::is_none")]
    pub standard: StandardEnum,
}




/// For standard client
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StandardEnum {
    Svg,
    None,
}

impl StandardEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Svg => "svg",
            Self::None => "none",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "svg" => Some(Self::Svg),
            "none" => Some(Self::None),
            _ => None,
        }
    }
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Available Deriv X all account types (Synthetic Indices and Financials).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct DxtradeAllCompany {
    /// Landing Company details.
    #[serde(rename = "standard", skip_serializing_if = "Option::is_none")]
    pub standard: Standard,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Landing Company details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Standard {
    /// Landing Company address
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<Value>,
    /// Special conditions for changing sensitive fields
    #[serde(rename = "changeable_fields", skip_serializing_if = "Option::is_none")]
    pub changeable_fields: ChangeableFields,
    /// Landing Company country of incorporation
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: String,
    /// The configuration of each currency.
    #[serde(rename = "currency_config", skip_serializing_if = "Option::is_none")]
    pub currency_config: CurrencyConfig,
    /// Flag to indicate whether reality check is applicable for this Landing Company. `1`: applicable, `0`: not applicable. The Reality Check is a feature that gives a summary of the client's trades and account balances on a regular basis throughout his session, and is a regulatory requirement for certain Landing Companies.
    #[serde(rename = "has_reality_check", skip_serializing_if = "Option::is_none")]
    pub has_reality_check: HasRealityCheckEnum,
    /// Allowed contract types
    #[serde(rename = "legal_allowed_contract_categories", skip_serializing_if = "Option::is_none")]
    pub legal_allowed_contract_categories: Vec<String>,
    /// Allowable currencies
    #[serde(rename = "legal_allowed_currencies", skip_serializing_if = "Option::is_none")]
    pub legal_allowed_currencies: Vec<String>,
    /// Allowable markets
    #[serde(rename = "legal_allowed_markets", skip_serializing_if = "Option::is_none")]
    pub legal_allowed_markets: Vec<String>,
    /// Default account currency
    #[serde(rename = "legal_default_currency", skip_serializing_if = "Option::is_none")]
    pub legal_default_currency: String,
    /// Landing Company legal name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Legal requirements for the Landing Company
    #[serde(rename = "requirements", skip_serializing_if = "Option::is_none")]
    pub requirements: Requirements,
    /// Landing Company short code
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


/// Available Deriv X financial account types (all except Synthetic Indices).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct DxtradeFinancialCompany {
    /// Landing Company details.
    #[serde(rename = "standard", skip_serializing_if = "Option::is_none")]
    pub standard: Standard,
}






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Available Deriv X derived account types (Synthetic Indices).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct DxtradeGamingCompany {
    /// Landing Company details.
    #[serde(rename = "standard", skip_serializing_if = "Option::is_none")]
    pub standard: Standard,
}






/// Flag to indicate if address parseable or not
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AddressParseableEnum {
    Value1 = 1,
    Value0,
}

impl AddressParseableEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Value1 => "1",
            Self::Value0 => "0",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "1" => Some(Self::Value1),
            "0" => Some(Self::Value0),
            _ => None,
        }
    }
}
/// Config for all account types (Synthetic Indices and Financials).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AllCompanyEnum {
    Svg,
    None,
}

impl AllCompanyEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Svg => "svg",
            Self::None => "none",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "svg" => Some(Self::Svg),
            "none" => Some(Self::None),
            _ => None,
        }
    }
}
/// Flag to indicate if idv is supported or not
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IsIdvSupportedEnum {
    Value1 = 1,
    Value0,
}

impl IsIdvSupportedEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Value1 => "1",
            Self::Value0 => "0",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "1" => Some(Self::Value1),
            "0" => Some(Self::Value0),
            _ => None,
        }
    }
}
/// Flag to indicate if mt5 age verification detail.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mt5AgeVerificationEnum {
    Value1 = 1,
    Value0,
}

impl Mt5AgeVerificationEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Value1 => "1",
            Self::Value0 => "0",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "1" => Some(Self::Value1),
            "0" => Some(Self::Value0),
            _ => None,
        }
    }
}
/// Flag to indicate whether max turnover limit settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NeedSetMaxTurnoverLimitEnum {
    Value0,
    Value1 = 1,
}

impl NeedSetMaxTurnoverLimitEnum {
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
/// Flag to indicate province settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoProvinceEnum {
    Value0,
    Value1 = 1,
}

impl NoProvinceEnum {
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
/// Flag to indicate whether address postcode is required or not.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequireAddressPostcodeEnum {
    Value0,
    Value1 = 1,
}

impl RequireAddressPostcodeEnum {
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
/// Flag to indicate whether age verification required ofr synthetic or not.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequireAgeVerifiedForSyntheticEnum {
    Value0,
    Value1 = 1,
}

impl RequireAgeVerifiedForSyntheticEnum {
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
/// Flag to indicate whether poi is required.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequirePoiEnum {
    Value0,
    Value1 = 1,
}

impl RequirePoiEnum {
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
/// Flag to indicate whether verification required if age not verified.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequireVerificationWhenNotAgeVerifiedEnum {
    Value0,
    Value1 = 1,
}

impl RequireVerificationWhenNotAgeVerifiedEnum {
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
/// Flag to indicate whether to skip deposit verifcation or not.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SkipDepositVerificationEnum {
    Value0,
    Value1 = 1,
}

impl SkipDepositVerificationEnum {
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


