
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/get_account_status/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 
use serde_json::Value;
use std::collections::HashMap;


// Import shared types from the *same* crate
use crate::authentication::Authentication; 
use crate::currency_config_value::CurrencyConfigValue; 
use crate::social_identity_provider_enum::SocialIdentityProviderEnum; 
use crate::prompt_client_to_authenticate_enum::PromptClientToAuthenticateEnum; 
use crate::p2p_status_enum::P2pStatusEnum; 
use crate::p2p_poa_required_enum::P2pPoaRequiredEnum; 

// It's a struct
/// Account status details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetAccountStatus {
    /// This represents the authentication status of the user and it includes what authentication is needed.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub authentication: Option<Authentication>,
    /// Contains missing profile fields required for cashier access.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub cashier_missing_fields: Option<Vec<String>>,
    /// If the cashier is unavailble, this array contains one or more error codes for each reason.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub cashier_validation: Option<Vec<String>>,
    /// Provides cashier details for client currency.\n
    // Correct serde attribute construction - Use helper
    
    pub currency_config: HashMap<String, CurrencyConfigValue>,
    /// P2P requires proof of address.\n
    // Correct serde attribute construction - Use helper
    
    pub p2p_poa_required: P2pPoaRequiredEnum,
    /// Current P2P status of client.\n
    // Correct serde attribute construction - Use helper
    
    pub p2p_status: P2pStatusEnum,
    /// Indicates whether the client should be prompted to authenticate their account.\n
    // Correct serde attribute construction - Use helper
    
    pub prompt_client_to_authenticate: PromptClientToAuthenticateEnum,
    /// Client risk classification: `low`, `standard`, `high`.\n
    // Correct serde attribute construction - Use helper
    
    pub risk_classification: String,
    /// Social identity provider a user signed up with.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub social_identity_provider: Option<SocialIdentityProviderEnum>,
    /// Account status. Possible status:\n/// - `address_verified`: client's address is verified by third party services.\n/// - `allow_document_upload`: client is allowed to upload documents.\n/// - `age_verification`: client is age-verified.\n/// - `authenticated`: client is fully authenticated.\n/// - `cashier_locked`: cashier is locked.\n/// - `crs_tin_information`: client has updated tax related information.\n/// - `deposit_locked`: deposit is not allowed.\n/// - `disabled`: account is disabled.\n/// - `document_expired`: client's submitted proof-of-identity documents have expired.\n/// - `document_expiring_soon`: client's submitted proof-of-identity documents are expiring within a month.\n/// - `dxtrade_password_not_set`: Deriv X password is not set.\n/// - `financial_assessment_not_complete`: client should complete their financial assessment.\n/// - `financial_information_not_complete`: client has not completed financial assessment.\n/// - `financial_risk_approval`: client has accepted financial risk disclosure.\n/// - `max_turnover_limit_not_set`: client has not set financial limits on their account. Applies to UK and Malta clients.\n/// - `mt5_password_not_set`: MT5 password is not set.\n/// - `mt5_withdrawal_locked`: MT5 deposits allowed, but withdrawal is not allowed.\n/// - `needs_affiliate_coc_approval`: user must approve the Affiliate's Code of Conduct Agreement.\n/// - `no_trading`: trading is disabled.\n/// - `no_withdrawal_or_trading`: client cannot trade or withdraw but can deposit.\n/// - `p2p_blocked_for_pa`: p2p is blocked for the current payment agent client.\n/// - `pa_withdrawal_explicitly_allowed`: withdrawal through payment agent is allowed.\n/// - `password_reset_required`: this client must reset their password.\n/// - `professional`: this client has opted for a professional account.\n/// - `professional_requested`: this client has requested for a professional account.\n/// - `professional_rejected`: this client's request for a professional account has been rejected.\n/// - `social_signup`: this client is using social signup.\n/// - `trading_experience_not_complete`: client has not completed the trading experience questionnaire.\n/// - `ukgc_funds_protection`: client has acknowledged UKGC funds protection notice.\n/// - `unwelcome`: client cannot deposit or buy contracts, but can withdraw or sell contracts.\n/// - `withdrawal_locked`: deposits allowed but withdrawals are not allowed.\n/// - `deposit_attempt`: this prevent a client from changing the account currency after deposit attempt.\n/// - `poi_name_mismatch`: client POI documents name mismatch.\n/// - `allow_poa_resubmission`: the client can resubmit POA documents.\n/// - `allow_poi_resubmission`: the client can resubmit POI documents.\n/// - `shared_payment_method`: the client has been sharing payment methods.\n/// - `personal_details_locked`: client is not allowed to edit personal profile details.\n/// - `transfers_blocked`: it block any transfer between two accounts.\n/// - `df_deposit_requires_poi`: the DF deposit will be blocked until the client gets age verified.\n/// - `authenticated_with_idv_photoid`: the client has been fully authenticated by IDV.\n/// - `idv_revoked`: the client used to be fully authenticated by IDV but it was taken away due to compliance criteria.\n/// - `mt5_additional_kyc_required`: client tax information, place of birth and account opening reason is missing.\n/// - `poi_expiring_soon`: the POI documents of the client will get expired soon, allow them to reupload POI documents.\n/// - `poa_authenticated_with_idv`: the POA is authenticated via IDV.\n/// - `poa_expiring_soon`: the POA documents of the client will get outdated soon, allow them to reupload POA documents.\n/// - `tin_manually_approved`: the client's tax_identification_number has been manually approved as client is not applicable for tax_identification_number\n
    // Correct serde attribute construction - Use helper
    
    pub status: Vec<String>,
}

