
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/partner_account_creation_status/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::create_eu_partner::CreateEuPartner; 
use crate::link_partner_eu::LinkPartnerEu; 
use crate::create_row_partner::CreateRowPartner; 
use crate::create_c_f_d_account::CreateCFDAccount; 
use crate::create_third_party_provider_user::CreateThirdPartyProviderUser; 
use crate::link_partner_row::LinkPartnerRow; 

// It's a struct
/// Status information for the partner account creation process
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PartnerAccountCreationStatus {
    /// Status of CFD account creation\n
    // Correct serde attribute construction - Use helper
    #[serde(rename = "create_CFD_account")] 
    pub create_cfd_account: CreateCFDAccount,
    /// Status of EU partner account creation\n
    // Correct serde attribute construction - Use helper
    
    pub create_eu_partner: CreateEuPartner,
    /// Status of ROW partner account creation\n
    // Correct serde attribute construction - Use helper
    
    pub create_row_partner: CreateRowPartner,
    /// Status of third party user creation\n
    // Correct serde attribute construction - Use helper
    
    pub create_third_party_provider_user: CreateThirdPartyProviderUser,
    /// Status of linking EU partner account\n
    // Correct serde attribute construction - Use helper
    
    pub link_partner_eu: LinkPartnerEu,
    /// Status of linking ROW partner account\n
    // Correct serde attribute construction - Use helper
    
    pub link_partner_row: LinkPartnerRow,
}

