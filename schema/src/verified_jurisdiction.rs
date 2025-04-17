
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/get_account_status/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::bvi_enum::BviEnum; 
use crate::virtual_enum::VirtualEnum; 
use crate::labuan_enum::LabuanEnum; 
use crate::svg_enum::SvgEnum; 
use crate::malta_enum::MaltaEnum; 
use crate::dml_enum::DmlEnum; 
use crate::dsl_enum::DslEnum; 
use crate::samoa_enum::SamoaEnum; 
use crate::samoa_virtual_enum::SamoaVirtualEnum; 
use crate::vanuatu_enum::VanuatuEnum; 
use crate::iom_enum::IomEnum; 
use crate::maltainvest_enum::MaltainvestEnum; 

// It's a struct
/// This represents the current status of authentication for each mt5 jurisdiction.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerifiedJurisdiction {
    /// This represents whether the client is allowed or not to create an account under this jurisdiction\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub bvi: Option<BviEnum>,
    /// This represents whether the client is allowed or not to create an account under this jurisdiction\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub dml: Option<DmlEnum>,
    /// This represents whether the client is allowed or not to create an account under this jurisdiction\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub dsl: Option<DslEnum>,
    /// This represents whether the client is allowed or not to create an account under this jurisdiction\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub iom: Option<IomEnum>,
    /// This represents whether the client is allowed or not to create an account under this jurisdiction\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub labuan: Option<LabuanEnum>,
    /// This represents whether the client is allowed or not to create an account under this jurisdiction\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub malta: Option<MaltaEnum>,
    /// This represents whether the client is allowed or not to create an account under this jurisdiction\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub maltainvest: Option<MaltainvestEnum>,
    /// This represents whether the client is allowed or not to create an account under this jurisdiction\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub samoa: Option<SamoaEnum>,
    /// This represents whether the client is allowed or not to create an account under this jurisdiction\n
    // Correct serde attribute construction - Use helper
    #[serde(rename = "samoa-virtual", skip_serializing_if = "Option::is_none")] 
    pub samoa_virtual: Option<SamoaVirtualEnum>,
    /// This represents whether the client is allowed or not to create an account under this jurisdiction\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub svg: Option<SvgEnum>,
    /// This represents whether the client is allowed or not to create an account under this jurisdiction\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub vanuatu: Option<VanuatuEnum>,
    /// This represents whether the client is allowed or not to create an account under this jurisdiction\n
    // Correct serde attribute construction - Use helper
    #[serde(rename = "virtual", skip_serializing_if = "Option::is_none")] 
    pub r#virtual: Option<VirtualEnum>,
}

