
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/mt5_login_list/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 
use serde_json::Value;



// Import shared types from the *same* crate
use crate::sub_account_category::SubAccountCategory; 
use crate::product::Product; 
use crate::sub_account_type::SubAccountType; 
use crate::server_info::ServerInfo; 
use crate::error::Error; 
use crate::white_label_links::WhiteLabelLinks; 
use crate::client_kyc_status::ClientKycStatus; 
use crate::landing_company_short::LandingCompanyShort; 
use crate::market_type::MarketType; 
use crate::account_type::AccountType; 
use crate::rights::Rights; 

// It's a struct
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Mt5LoginListItem {
    /// Account type.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub account_type: Option<AccountType>,
    /// Balance of the MT5 account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub balance: Option<f64>,
    /// [Optional] Pertains to client KYC. Returned only if the client fails to meet the requirements, including proof of identity (POI), validity of the tax identification number (TIN), and proof of address (POA).\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub client_kyc_status: Option<ClientKycStatus>,
    /// Residence of the MT5 account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub country: Option<String>,
    /// Currency of the MT5 account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub currency: Option<String>,
    /// Account balance, formatted to appropriate decimal places.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub display_balance: Option<String>,
    /// [Optional] Determines the eligibility status for migrating a client account based on verification and account type.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub eligible_to_migrate: Option<Value>,
    /// Email address of the MT5 account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub email: Option<String>,
    /// Error in MT5 account details.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub error: Option<Error>,
    /// Group type of the MT5 account, e.g. `demo\svg_financial`\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub group: Option<String>,
    /// Indicate if the account is a main agent - an IB account\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub is_main_agent: Option<bool>,
    /// Broker name\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub landing_company: Option<String>,
    /// Landing company shortcode of the MT5 account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub landing_company_short: Option<LandingCompanyShort>,
    /// Leverage of the MT5 account (1 to 1000).\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub leverage: Option<f64>,
    /// Login of MT5 account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub login: Option<String>,
    /// Market type\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub market_type: Option<MarketType>,
    /// Name of the owner of the MT5 account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub name: Option<String>,
    /// Product name that Deriv offer\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub product: Option<Product>,
    /// Timestamp of the latest MT5 request.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub request_timestamp: Option<i64>,
    /// Rights assigned to the MT5 account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub rights: Option<Rights>,
    /// Trade server name of the MT5 account.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub server: Option<String>,
    /// Trade server information.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub server_info: Option<ServerInfo>,
    /// MT5 account status.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub status: Option<Value>,
    /// Sub account category refer to the additional risk management\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub sub_account_category: Option<SubAccountCategory>,
    /// Sub account type refer to different offerings that we have for mt5\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub sub_account_type: Option<SubAccountType>,
    /// MT5 webtrader url for each mt5 platform\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub webtrader_url: Option<String>,
    /// Links to access MT5 application for different platforms.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub white_label_links: Option<WhiteLabelLinks>,
}

