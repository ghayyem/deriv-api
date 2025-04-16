
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/landing_company/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 
use serde_json::Value;
use std::collections::HashMap;


// Import shared types from the *same* crate
use crate::require_address_postcode_enum::RequireAddressPostcodeEnum; 
use crate::ctrader::Ctrader; 
use crate::require_verification_when_not_age_verified_enum::RequireVerificationWhenNotAgeVerifiedEnum; 
use crate::skip_deposit_verification_enum::SkipDepositVerificationEnum; 
use crate::mt5_age_verification_enum::Mt5AgeVerificationEnum; 
use crate::all_company_enum::AllCompanyEnum; 
use crate::no_province_enum::NoProvinceEnum; 
use crate::is_idv_supported_enum::IsIdvSupportedEnum; 
use crate::need_set_max_turnover_limit_enum::NeedSetMaxTurnoverLimitEnum; 
use crate::dxtrade_gaming_company::DxtradeGamingCompany; 
use crate::address_parseable_enum::AddressParseableEnum; 
use crate::dxtrade_all_company::DxtradeAllCompany; 
use crate::dxtrade_financial_company::DxtradeFinancialCompany; 
use crate::require_age_verified_for_synthetic_enum::RequireAgeVerifiedForSyntheticEnum; 
use crate::require_poi_enum::RequirePoiEnum; 

// It's a struct
/// Landing Company
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LandingCompany {
    /// Flag to indicate if address parseable or not\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub address_parseable: Option<AddressParseableEnum>,
    /// Config for all account types (Synthetic Indices and Financials).\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub all_company: Option<AllCompanyEnum>,
    /// Config structure with document types ,taxRequired ,tin format details.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub config: Option<Value>,
    /// Available CTrader accounts.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub ctrader: Option<Ctrader>,
    /// Available Deriv X all account types (Synthetic Indices and Financials).\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub dxtrade_all_company: Option<DxtradeAllCompany>,
    /// Available Deriv X financial account types (all except Synthetic Indices).\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub dxtrade_financial_company: Option<DxtradeFinancialCompany>,
    /// Available Deriv X derived account types (Synthetic Indices).\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub dxtrade_gaming_company: Option<DxtradeGamingCompany>,
    /// Landing Company for financial contracts (all except Synthetic Indices)\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub financial_company: Option<Value>,
    /// Forbidden postcode pattern\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub forbidden_postcode_pattern: Option<String>,
    /// Landing Company for derived contracts (Synthetic Indices)\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub gaming_company: Option<Value>,
    /// Country code\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub id: Option<String>,
    /// Flag to indicate if idv is supported or not\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub is_idv_supported: Option<IsIdvSupportedEnum>,
    /// Open mf account lc details.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub lc_to_open_mf_account: Option<String>,
    /// Minimum age\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub minimum_age: Option<i64>,
    /// Flag to indicate if mt5 age verification detail.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub mt5_age_verification: Option<Mt5AgeVerificationEnum>,
    /// Landing Company for MT5 standard combined all Synthetic and financial, currently has Financial as subtype.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub mt_all_company: Option<Value>,
    /// Landing Company for MT5 financial contracts (all except Synthetic Indices), currently divided into Financial STP, Financial (standard) as subtypes.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub mt_financial_company: Option<Value>,
    /// Landing Company for MT5 standard derived contracts (Synthetic Indices), currently has Financial as subtype.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub mt_gaming_company: Option<Value>,
    /// Country name\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub name: Option<String>,
    /// Flag to indicate whether max turnover limit settings.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub need_set_max_turnover_limit: Option<NeedSetMaxTurnoverLimitEnum>,
    /// Flag to indicate province settings.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub no_province: Option<NoProvinceEnum>,
    /// Flag to indicate whether address postcode is required or not.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub require_address_postcode: Option<RequireAddressPostcodeEnum>,
    /// Flag to indicate whether age verification required ofr synthetic or not.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub require_age_verified_for_synthetic: Option<RequireAgeVerifiedForSyntheticEnum>,
    /// Flag to indicate whether poi is required.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub require_poi: Option<RequirePoiEnum>,
    /// Flag to indicate whether verification required if age not verified.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub require_verification_when_not_age_verified: Option<RequireVerificationWhenNotAgeVerifiedEnum>,
    /// Flag to indicate whether to skip deposit verifcation or not.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub skip_deposit_verification: Option<SkipDepositVerificationEnum>,
    /// Virtual Company\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub virtual_company: Option<String>,
}

