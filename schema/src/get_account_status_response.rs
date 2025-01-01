
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/get_account_status/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// A message with Account Status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct GetAccountStatusResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Account status details
    #[serde(rename = "get_account_status", skip_serializing_if = "Option::is_none")]
    pub get_account_status: GetAccountStatus,
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


/// Account status details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct GetAccountStatus {
    /// This represents the authentication status of the user and it includes what authentication is needed.
    #[serde(rename = "authentication", skip_serializing_if = "Option::is_none")]
    pub authentication: Authentication,
    /// Contains missing profile fields required for cashier access.
    #[serde(rename = "cashier_missing_fields", skip_serializing_if = "Option::is_none")]
    pub cashier_missing_fields: Vec<String>,
    /// If the cashier is unavailble, this array contains one or more error codes for each reason.
    #[serde(rename = "cashier_validation", skip_serializing_if = "Option::is_none")]
    pub cashier_validation: Vec<String>,
    /// Provides cashier details for client currency.
    #[serde(rename = "currency_config", flatten)]
    pub currency_config: HashMap<String, CurrencyConfigvalue>,
    /// P2P requires proof of address.
    #[serde(rename = "p2p_poa_required")]
    pub p_2p_poa_required: P2pPoaRequiredEnum,
    /// Current P2P status of client.
    #[serde(rename = "p2p_status")]
    pub p_2p_status: P2pStatusEnum,
    /// Indicates whether the client should be prompted to authenticate their account.
    #[serde(rename = "prompt_client_to_authenticate")]
    pub prompt_client_to_authenticate: PromptClientToAuthenticateEnum,
    /// Client risk classification: `low`, `standard`, `high`.
    #[serde(rename = "risk_classification")]
    pub risk_classification: String,
    /// Social identity provider a user signed up with.
    #[serde(rename = "social_identity_provider", skip_serializing_if = "Option::is_none")]
    pub social_identity_provider: SocialIdentityProviderEnum,
    /// Account status. Possible status: 
/// - `address_verified`: client's address is verified by third party services. 
/// - `allow_document_upload`: client is allowed to upload documents. 
/// - `age_verification`: client is age-verified. 
/// - `authenticated`: client is fully authenticated. 
/// - `cashier_locked`: cashier is locked. 
/// - `crs_tin_information`: client has updated tax related information. 
/// - `deposit_locked`: deposit is not allowed. 
/// - `disabled`: account is disabled. 
/// - `document_expired`: client's submitted proof-of-identity documents have expired. 
/// - `document_expiring_soon`: client's submitted proof-of-identity documents are expiring within a month. 
/// - `dxtrade_password_not_set`: Deriv X password is not set. 
/// - `financial_assessment_not_complete`: client should complete their financial assessment. 
/// - `financial_information_not_complete`: client has not completed financial assessment. 
/// - `financial_risk_approval`: client has accepted financial risk disclosure. 
/// - `max_turnover_limit_not_set`: client has not set financial limits on their account. Applies to UK and Malta clients. 
/// - `mt5_password_not_set`: MT5 password is not set. 
/// - `mt5_withdrawal_locked`: MT5 deposits allowed, but withdrawal is not allowed. 
/// - `needs_affiliate_coc_approval`: user must approve the Affiliate's Code of Conduct Agreement. 
/// - `no_trading`: trading is disabled. 
/// - `no_withdrawal_or_trading`: client cannot trade or withdraw but can deposit. 
/// - `p2p_blocked_for_pa`: p2p is blocked for the current payment agent client. 
/// - `pa_withdrawal_explicitly_allowed`: withdrawal through payment agent is allowed. 
/// - `password_reset_required`: this client must reset their password.  
/// - `professional`: this client has opted for a professional account. 
/// - `professional_requested`: this client has requested for a professional account. 
/// - `professional_rejected`: this client's request for a professional account has been rejected. 
/// - `social_signup`: this client is using social signup. 
/// - `trading_experience_not_complete`: client has not completed the trading experience questionnaire. 
/// - `ukgc_funds_protection`: client has acknowledged UKGC funds protection notice. 
/// - `unwelcome`: client cannot deposit or buy contracts, but can withdraw or sell contracts. 
/// - `withdrawal_locked`: deposits allowed but withdrawals are not allowed. 
/// - `deposit_attempt`: this prevent a client from changing the account currency after deposit attempt. 
/// - `poi_name_mismatch`: client POI documents name mismatch. 
/// - `allow_poa_resubmission`: the client can resubmit POA documents. 
/// - `allow_poi_resubmission`: the client can resubmit POI documents. 
/// - `shared_payment_method`: the client has been sharing payment methods. 
/// - `personal_details_locked`: client is not allowed to edit personal profile details. 
/// - `transfers_blocked`: it block any transfer between two accounts. 
/// - `df_deposit_requires_poi`: the DF deposit will be blocked until the client gets age verified. 
/// - `authenticated_with_idv_photoid`: the client has been fully authenticated by IDV. 
/// - `idv_revoked`: the client used to be fully authenticated by IDV but it was taken away due to compliance criteria. 
/// - `mt5_additional_kyc_required`: client tax information, place of birth and account opening reason is missing. 
/// - `poi_expiring_soon`: the POI documents of the client will get expired soon, allow them to reupload POI documents. 
/// - `poa_authenticated_with_idv`: the POA is authenticated via IDV. 
/// - `poa_expiring_soon`: the POA documents of the client will get outdated soon, allow them to reupload POA documents. 
/// - `tin_manually_approved`: the client's tax_identification_number has been manually approved as client is not applicable for tax_identification_number
    #[serde(rename = "status")]
    pub status: Vec<String>,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// This represents the authentication status of the user and it includes what authentication is needed.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Authentication {
    /// POI attempts made by the client
    #[serde(rename = "attempts", skip_serializing_if = "Option::is_none")]
    pub attempts: Attempts,
    /// The authentication status for document.
    #[serde(rename = "document", skip_serializing_if = "Option::is_none")]
    pub document: Document,
    /// The authentication status for identity.
    #[serde(rename = "identity", skip_serializing_if = "Option::is_none")]
    pub identity: Identity,
    /// The authentication status for source of income document.
    #[serde(rename = "income", skip_serializing_if = "Option::is_none")]
    pub income: Income,
    /// An array containing the list of required authentication.
    #[serde(rename = "needs_verification")]
    pub needs_verification: Vec<String>,
    /// The current state of the proof of ownership.
    #[serde(rename = "ownership", skip_serializing_if = "Option::is_none")]
    pub ownership: Ownership,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// POI attempts made by the client
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Attempts {
    /// A number of POI attempts made by the client
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: i64,
    /// A list of POI attempts made by the client in chronological descending order
    #[serde(rename = "history", skip_serializing_if = "Option::is_none")]
    pub history: Vec<Historyitem>,
    /// The latest POI attempt made by the client
    #[serde(rename = "latest", skip_serializing_if = "Option::is_none")]
    pub latest: Option<Value>,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;

use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Historyitem {
    /// 2-letter country code used to request the attempt.
    #[serde(rename = "country_code", skip_serializing_if = "Option::is_none")]
    pub country_code: String,
    /// The document type of the attempt.
    #[serde(rename = "document_type", skip_serializing_if = "Option::is_none")]
    pub document_type: String,
    /// The id of the attempt.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    /// The service used to make the verification.
    #[serde(rename = "service", skip_serializing_if = "Option::is_none")]
    pub service: String,
    /// Status of the attempt.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: StatusEnum,
    /// The epoch of the attempt.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: i64,
}








// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// The authentication status for document.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Document {
    /// This is the epoch of the document expiry date.
    #[serde(rename = "expiry_date", skip_serializing_if = "Option::is_none")]
    pub expiry_date: i64,
    /// Show the last reported reasons for the rejected poa cases
    #[serde(rename = "last_rejected", skip_serializing_if = "Option::is_none")]
    pub last_rejected: Option<Value>,
    /// This represents the current status of the proof of address document submitted for authentication.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: StatusEnum,
    /// This represents the current status of authentication for each mt5 jurisdiction.
    #[serde(rename = "verified_jurisdiction", skip_serializing_if = "Option::is_none")]
    pub verified_jurisdiction: VerifiedJurisdiction,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// This represents the current status of authentication for each mt5 jurisdiction.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct VerifiedJurisdiction {
    /// This represents whether the client is allowed or not to create an account under this jurisdiction
    #[serde(rename = "bvi", skip_serializing_if = "Option::is_none")]
    pub bvi: BviEnum,
    /// This represents whether the client is allowed or not to create an account under this jurisdiction
    #[serde(rename = "dml", skip_serializing_if = "Option::is_none")]
    pub dml: DmlEnum,
    /// This represents whether the client is allowed or not to create an account under this jurisdiction
    #[serde(rename = "dsl", skip_serializing_if = "Option::is_none")]
    pub dsl: DslEnum,
    /// This represents whether the client is allowed or not to create an account under this jurisdiction
    #[serde(rename = "iom", skip_serializing_if = "Option::is_none")]
    pub iom: IomEnum,
    /// This represents whether the client is allowed or not to create an account under this jurisdiction
    #[serde(rename = "labuan", skip_serializing_if = "Option::is_none")]
    pub labuan: LabuanEnum,
    /// This represents whether the client is allowed or not to create an account under this jurisdiction
    #[serde(rename = "malta", skip_serializing_if = "Option::is_none")]
    pub malta: MaltaEnum,
    /// This represents whether the client is allowed or not to create an account under this jurisdiction
    #[serde(rename = "maltainvest", skip_serializing_if = "Option::is_none")]
    pub maltainvest: MaltainvestEnum,
    /// This represents whether the client is allowed or not to create an account under this jurisdiction
    #[serde(rename = "samoa", skip_serializing_if = "Option::is_none")]
    pub samoa: SamoaEnum,
    /// This represents whether the client is allowed or not to create an account under this jurisdiction
    #[serde(rename = "samoa-virtual", skip_serializing_if = "Option::is_none")]
    pub samoa-virtual: Samoa-virtualEnum,
    /// This represents whether the client is allowed or not to create an account under this jurisdiction
    #[serde(rename = "svg", skip_serializing_if = "Option::is_none")]
    pub svg: SvgEnum,
    /// This represents whether the client is allowed or not to create an account under this jurisdiction
    #[serde(rename = "vanuatu", skip_serializing_if = "Option::is_none")]
    pub vanuatu: VanuatuEnum,
    /// This represents whether the client is allowed or not to create an account under this jurisdiction
    #[serde(rename = "virtual", skip_serializing_if = "Option::is_none")]
    pub virtual: VirtualEnum,
}




/// This represents whether the client is allowed or not to create an account under this jurisdiction
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BviEnum {
    Value0,
    Value1 = 1,
}

impl BviEnum {
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
/// This represents whether the client is allowed or not to create an account under this jurisdiction
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DmlEnum {
    Value0,
    Value1 = 1,
}

impl DmlEnum {
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
/// This represents whether the client is allowed or not to create an account under this jurisdiction
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DslEnum {
    Value0,
    Value1 = 1,
}

impl DslEnum {
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
/// This represents whether the client is allowed or not to create an account under this jurisdiction
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IomEnum {
    Value0,
    Value1 = 1,
}

impl IomEnum {
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
/// This represents whether the client is allowed or not to create an account under this jurisdiction
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LabuanEnum {
    Value0,
    Value1 = 1,
}

impl LabuanEnum {
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
/// This represents whether the client is allowed or not to create an account under this jurisdiction
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MaltaEnum {
    Value0,
    Value1 = 1,
}

impl MaltaEnum {
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
/// This represents whether the client is allowed or not to create an account under this jurisdiction
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MaltainvestEnum {
    Value0,
    Value1 = 1,
}

impl MaltainvestEnum {
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
/// This represents whether the client is allowed or not to create an account under this jurisdiction
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SamoaEnum {
    Value0,
    Value1 = 1,
}

impl SamoaEnum {
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
/// This represents whether the client is allowed or not to create an account under this jurisdiction
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Samoa-virtualEnum {
    Value0,
    Value1 = 1,
}

impl Samoa-virtualEnum {
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
/// This represents whether the client is allowed or not to create an account under this jurisdiction
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SvgEnum {
    Value0,
    Value1 = 1,
}

impl SvgEnum {
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
/// This represents whether the client is allowed or not to create an account under this jurisdiction
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VanuatuEnum {
    Value0,
    Value1 = 1,
}

impl VanuatuEnum {
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
/// This represents whether the client is allowed or not to create an account under this jurisdiction
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VirtualEnum {
    Value0,
    Value1 = 1,
}

impl VirtualEnum {
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




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// The authentication status for identity.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Identity {
    /// This is the epoch of the document expiry date.
    #[serde(rename = "expiry_date", skip_serializing_if = "Option::is_none")]
    pub expiry_date: i64,
    /// This shows the information about the authentication services implemented
    #[serde(rename = "services", skip_serializing_if = "Option::is_none")]
    pub services: Services,
    /// This represent the current status for proof of identity document submitted for authentication.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: StatusEnum,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// This shows the information about the authentication services implemented
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Services {
    /// This shows the information related to IDV supported services
    #[serde(rename = "idv", skip_serializing_if = "Option::is_none")]
    pub idv: Idv,
    /// This shows the information related to the manual POI checks
    #[serde(rename = "manual", skip_serializing_if = "Option::is_none")]
    pub manual: Manual,
    /// This shows the information related to Onfido supported services
    #[serde(rename = "onfido", skip_serializing_if = "Option::is_none")]
    pub onfido: Onfido,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// This shows the information related to IDV supported services
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Idv {
    /// This is the epoch of the document expiry date.
    #[serde(rename = "expiry_date", skip_serializing_if = "Option::is_none")]
    pub expiry_date: i64,
    /// Show the last IDV reported reasons for the rejected cases
    #[serde(rename = "last_rejected", skip_serializing_if = "Option::is_none")]
    pub last_rejected: Vec<String>,
    /// Indicate if the verification report was returned by the provider
    #[serde(rename = "report_available", skip_serializing_if = "Option::is_none")]
    pub report_available: ReportAvailableEnum,
    /// Shows the latest document properties detected and reported by IDVS
    #[serde(rename = "reported_properties", skip_serializing_if = "Option::is_none")]
    pub reported_properties: ReportedProperties,
    /// This represents the status of the latest IDV check.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: StatusEnum,
    /// This shows the number of IDV submissions left for the client
    #[serde(rename = "submissions_left", skip_serializing_if = "Option::is_none")]
    pub submissions_left: i64,
}




/// Indicate if the verification report was returned by the provider
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportAvailableEnum {
    Value0,
    Value1 = 1,
}

impl ReportAvailableEnum {
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


// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// This shows the information related to the manual POI checks
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Manual {
    /// This represents the status of the current manual POI check.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: StatusEnum,
}






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// This shows the information related to Onfido supported services
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Onfido {
    /// 3 letter country code for Onfide SDK
    #[serde(rename = "country_code", skip_serializing_if = "Option::is_none")]
    pub country_code: String,
    /// This shows the list of documents types supported by Onfido
    #[serde(rename = "documents", skip_serializing_if = "Option::is_none")]
    pub documents: Vec<String>,
    /// This shows the list of documents types supported.
    #[serde(rename = "documents_supported", skip_serializing_if = "Option::is_none")]
    pub documents_supported: Vec<String>,
    /// This shows the information if the country is supported by Onfido
    #[serde(rename = "is_country_supported", skip_serializing_if = "Option::is_none")]
    pub is_country_supported: IsCountrySupportedEnum,
    /// Show the last Onfido reported reasons for the rejected cases
    #[serde(rename = "last_rejected", skip_serializing_if = "Option::is_none")]
    pub last_rejected: Vec<String>,
    /// Shows the latest document properties detected and reported by Onfido
    #[serde(rename = "reported_properties", skip_serializing_if = "Option::is_none")]
    pub reported_properties: ReportedProperties,
    /// This represents the status of the latest Onfido check.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: StatusEnum,
    /// This shows the number of Onfido submissions left for the client
    #[serde(rename = "submissions_left", skip_serializing_if = "Option::is_none")]
    pub submissions_left: i64,
}




/// This shows the information if the country is supported by Onfido
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IsCountrySupportedEnum {
    Value1 = 1,
    Value0,
}

impl IsCountrySupportedEnum {
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






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// The authentication status for source of income document.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Income {
    /// Epoch of the source of income document expiry date.
    #[serde(rename = "expiry_date", skip_serializing_if = "Option::is_none")]
    pub expiry_date: i64,
    /// Current status of the proof of income document submitted for authentication.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: StatusEnum,
}






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// The current state of the proof of ownership.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Ownership {
    /// The list of proof of ownership requests to fullfil
    #[serde(rename = "requests", skip_serializing_if = "Option::is_none")]
    pub requests: Vec<Requestsitem>,
    /// This represents the current status of the proof of ownership
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: StatusEnum,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Requestsitem {
    /// The request timestamp of creation
    #[serde(rename = "creation_time", skip_serializing_if = "Option::is_none")]
    pub creation_time: String,
    /// Number of documents required to be uploaded for proof of ownership
    #[serde(rename = "documents_required", skip_serializing_if = "Option::is_none")]
    pub documents_required: f64,
    /// The identifier of the proof of ownership request
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: f64,
    /// The display name of the payment method being requested
    #[serde(rename = "payment_method", skip_serializing_if = "Option::is_none")]
    pub payment_method: String,
}










// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Client currency
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct CurrencyConfigvalue {
    /// Deposit is allowed for currency or not
    #[serde(rename = "is_deposit_suspended", skip_serializing_if = "Option::is_none")]
    pub is_deposit_suspended: IsDepositSuspendedEnum,
    /// Withdrawal is allowed for currency or not
    #[serde(rename = "is_withdrawal_suspended", skip_serializing_if = "Option::is_none")]
    pub is_withdrawal_suspended: IsWithdrawalSuspendedEnum,
}




/// Deposit is allowed for currency or not
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IsDepositSuspendedEnum {
    Value0,
    Value1 = 1,
}

impl IsDepositSuspendedEnum {
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
/// Withdrawal is allowed for currency or not
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IsWithdrawalSuspendedEnum {
    Value0,
    Value1 = 1,
}

impl IsWithdrawalSuspendedEnum {
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


/// P2P requires proof of address.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum P2pPoaRequiredEnum {
    Value0,
    Value1 = 1,
}

impl P2pPoaRequiredEnum {
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
/// Current P2P status of client.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum P2pStatusEnum {
    None,
    Active,
    Temp_Ban,
    Perm_Ban,
}

impl P2pStatusEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Active => "active",
            Self::Temp_Ban => "temp_ban",
            Self::Perm_Ban => "perm_ban",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "none" => Some(Self::None),
            "active" => Some(Self::Active),
            "temp_ban" => Some(Self::Temp_Ban),
            "perm_ban" => Some(Self::Perm_Ban),
            _ => None,
        }
    }
}
/// Indicates whether the client should be prompted to authenticate their account.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromptClientToAuthenticateEnum {
    Value1 = 1,
    Value0,
}

impl PromptClientToAuthenticateEnum {
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
/// Social identity provider a user signed up with.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SocialIdentityProviderEnum {
    Google,
    Facebook,
    Apple,
}

impl SocialIdentityProviderEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Google => "google",
            Self::Facebook => "facebook",
            Self::Apple => "apple",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "google" => Some(Self::Google),
            "facebook" => Some(Self::Facebook),
            "apple" => Some(Self::Apple),
            _ => None,
        }
    }
}


