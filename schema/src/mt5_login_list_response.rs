
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/mt5_login_list/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Get list of MT5 accounts for client.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Mt5LoginListResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Array containing MT5 account objects.
    #[serde(rename = "mt5_login_list", skip_serializing_if = "Option::is_none")]
    pub mt_5_login_list: Vec<Mt5LoginListitem>,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Mt5LoginListitem {
    /// Account type.
    #[serde(rename = "account_type", skip_serializing_if = "Option::is_none")]
    pub account_type: AccountTypeEnum,
    /// Balance of the MT5 account.
    #[serde(rename = "balance", skip_serializing_if = "Option::is_none")]
    pub balance: f64,
    /// [Optional] Pertains to client KYC. Returned only if the client fails to meet the requirements, including proof of identity (POI), validity of the tax identification number (TIN), and proof of address (POA).
    #[serde(rename = "client_kyc_status", skip_serializing_if = "Option::is_none")]
    pub client_kyc_status: ClientKycStatus,
    /// Residence of the MT5 account.
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: String,
    /// Currency of the MT5 account.
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: String,
    /// Account balance, formatted to appropriate decimal places.
    #[serde(rename = "display_balance", skip_serializing_if = "Option::is_none")]
    pub display_balance: String,
    /// [Optional] Determines the eligibility status for migrating a client account based on verification and account type.
    #[serde(rename = "eligible_to_migrate", skip_serializing_if = "Option::is_none")]
    pub eligible_to_migrate: EligibleToMigrate,
    /// Email address of the MT5 account.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: String,
    /// Error in MT5 account details.
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Error,
    /// Group type of the MT5 account, e.g. `demo\svg_financial`
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: String,
    /// Indicate if the account is a main agent - an IB account
    #[serde(rename = "is_main_agent", skip_serializing_if = "Option::is_none")]
    pub is_main_agent: bool,
    /// Broker name
    #[serde(rename = "landing_company", skip_serializing_if = "Option::is_none")]
    pub landing_company: String,
    /// Landing company shortcode of the MT5 account.
    #[serde(rename = "landing_company_short", skip_serializing_if = "Option::is_none")]
    pub landing_company_short: LandingCompanyShortEnum,
    /// Leverage of the MT5 account (1 to 1000).
    #[serde(rename = "leverage", skip_serializing_if = "Option::is_none")]
    pub leverage: f64,
    /// Login of MT5 account.
    #[serde(rename = "login", skip_serializing_if = "Option::is_none")]
    pub login: String,
    /// Market type
    #[serde(rename = "market_type", skip_serializing_if = "Option::is_none")]
    pub market_type: MarketTypeEnum,
    /// Name of the owner of the MT5 account.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Product name that Deriv offer
    #[serde(rename = "product", skip_serializing_if = "Option::is_none")]
    pub product: ProductEnum,
    /// Timestamp of the latest MT5 request.
    #[serde(rename = "request_timestamp", skip_serializing_if = "Option::is_none")]
    pub request_timestamp: i64,
    /// Rights assigned to the MT5 account.
    #[serde(rename = "rights", skip_serializing_if = "Option::is_none")]
    pub rights: Rights,
    /// Trade server name of the MT5 account.
    #[serde(rename = "server", skip_serializing_if = "Option::is_none")]
    pub server: String,
    /// Trade server information.
    #[serde(rename = "server_info", skip_serializing_if = "Option::is_none")]
    pub server_info: ServerInfo,
    /// MT5 account status.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Value>,
    /// Sub account category refer to the additional risk management
    #[serde(rename = "sub_account_category", skip_serializing_if = "Option::is_none")]
    pub sub_account_category: SubAccountCategoryEnum,
    /// Sub account type refer to different offerings that we have for mt5
    #[serde(rename = "sub_account_type", skip_serializing_if = "Option::is_none")]
    pub sub_account_type: SubAccountTypeEnum,
    /// MT5 webtrader url for each mt5 platform
    #[serde(rename = "webtrader_url", skip_serializing_if = "Option::is_none")]
    pub webtrader_url: String,
    /// Links to access MT5 application for different platforms.
    #[serde(rename = "white_label_links", skip_serializing_if = "Option::is_none")]
    pub white_label_links: WhiteLabelLinks,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// [Optional] Pertains to client KYC. Returned only if the client fails to meet the requirements, including proof of identity (POI), validity of the tax identification number (TIN), and proof of address (POA).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct ClientKycStatus {
    /// Status of proof of address (POA).
    #[serde(rename = "poa_status", skip_serializing_if = "Option::is_none")]
    pub poa_status: PoaStatusEnum,
    /// Status of proof of identity (POI).
    #[serde(rename = "poi_status", skip_serializing_if = "Option::is_none")]
    pub poi_status: PoiStatusEnum,
    /// Indicates whether the tax identification number (TIN) is valid (1) or not (0).
    #[serde(rename = "valid_tin", skip_serializing_if = "Option::is_none")]
    pub valid_tin: ValidTinEnum,
}




/// Status of proof of address (POA).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PoaStatusEnum {
    None,
    Pending,
    Expired,
    Verified,
    Rejected,
}

impl PoaStatusEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Pending => "pending",
            Self::Expired => "expired",
            Self::Verified => "verified",
            Self::Rejected => "rejected",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "none" => Some(Self::None),
            "pending" => Some(Self::Pending),
            "expired" => Some(Self::Expired),
            "verified" => Some(Self::Verified),
            "rejected" => Some(Self::Rejected),
            _ => None,
        }
    }
}
/// Status of proof of identity (POI).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PoiStatusEnum {
    None,
    Pending,
    Verified,
    Suspected,
    Rejected,
    Expired,
}

impl PoiStatusEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Pending => "pending",
            Self::Verified => "verified",
            Self::Suspected => "suspected",
            Self::Rejected => "rejected",
            Self::Expired => "expired",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "none" => Some(Self::None),
            "pending" => Some(Self::Pending),
            "verified" => Some(Self::Verified),
            "suspected" => Some(Self::Suspected),
            "rejected" => Some(Self::Rejected),
            "expired" => Some(Self::Expired),
            _ => None,
        }
    }
}
/// Indicates whether the tax identification number (TIN) is valid (1) or not (0).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidTinEnum {
    Value1 = 1,
    Value0,
}

impl ValidTinEnum {
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
use std::collections::HashMap;


/// Error in MT5 account details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Error {
    /// Error code string.
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: String,
    /// Extra information about the error.
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Details,
    /// Error message.
    #[serde(rename = "message_to_client", skip_serializing_if = "Option::is_none")]
    pub message_to_client: String,
}






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Rights assigned to the MT5 account.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Rights {
    /// User is allowed to connect via MT5 Web API
    #[serde(rename = "api", skip_serializing_if = "Option::is_none")]
    pub api: bool,
    /// This flag is obsolete and not used
    #[serde(rename = "api_deprecated", skip_serializing_if = "Option::is_none")]
    pub api_deprecated: bool,
    /// User's certificate is confirmed
    #[serde(rename = "confirmed", skip_serializing_if = "Option::is_none")]
    pub confirmed: bool,
    /// The User is allowed to connect
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: bool,
    /// User is not allowed to view reports
    #[serde(rename = "exclude_reports", skip_serializing_if = "Option::is_none")]
    pub exclude_reports: bool,
    /// User is allowed to use Expert Advisors
    #[serde(rename = "expert", skip_serializing_if = "Option::is_none")]
    pub expert: bool,
    /// For internal mt5 usage
    #[serde(rename = "investor", skip_serializing_if = "Option::is_none")]
    pub investor: bool,
    /// User is allowed to use OTP
    #[serde(rename = "otp_enabled", skip_serializing_if = "Option::is_none")]
    pub otp_enabled: bool,
    /// User is allowed to change password
    #[serde(rename = "password_change", skip_serializing_if = "Option::is_none")]
    pub password_change: bool,
    /// User has enabled push notifications
    #[serde(rename = "push", skip_serializing_if = "Option::is_none")]
    pub push: bool,
    /// Value for internal mt5 usage
    #[serde(rename = "readonly", skip_serializing_if = "Option::is_none")]
    pub readonly: bool,
    /// User is allowed to receive daily reports
    #[serde(rename = "reports", skip_serializing_if = "Option::is_none")]
    pub reports: bool,
    /// User must change password during next connection
    #[serde(rename = "reset_pass", skip_serializing_if = "Option::is_none")]
    pub reset_pass: bool,
    /// VPS is enabled for user
    #[serde(rename = "sponsored", skip_serializing_if = "Option::is_none")]
    pub sponsored: bool,
    /// User can view technical accounts in manager/admin terminal
    #[serde(rename = "technical", skip_serializing_if = "Option::is_none")]
    pub technical: bool,
    /// Trading is disabled for user
    #[serde(rename = "trade_disabled", skip_serializing_if = "Option::is_none")]
    pub trade_disabled: bool,
    /// User is allowed to use trailing stops
    #[serde(rename = "trailing", skip_serializing_if = "Option::is_none")]
    pub trailing: bool,
}






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Links to access MT5 application for different platforms.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct WhiteLabelLinks {
    /// Download link for Android.
    #[serde(rename = "android", skip_serializing_if = "Option::is_none")]
    pub android: String,
    /// Download link for iOS.
    #[serde(rename = "ios", skip_serializing_if = "Option::is_none")]
    pub ios: String,
    /// MT5 webtrader url based on jurisdiction and platform
    #[serde(rename = "webtrader_url", skip_serializing_if = "Option::is_none")]
    pub webtrader_url: String,
    /// Download link for Windows.
    #[serde(rename = "windows", skip_serializing_if = "Option::is_none")]
    pub windows: String,
}






/// Product name that Deriv offer
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProductEnum {
    Value,
    Synthetic,
    Financial,
    Swap_Free,
    Zero_Spread,
    Standard,
    Stp,
    Gold,
}

impl ProductEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Value => "",
            Self::Synthetic => "synthetic",
            Self::Financial => "financial",
            Self::Swap_Free => "swap_free",
            Self::Zero_Spread => "zero_spread",
            Self::Standard => "standard",
            Self::Stp => "stp",
            Self::Gold => "gold",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "" => Some(Self::Value),
            "synthetic" => Some(Self::Synthetic),
            "financial" => Some(Self::Financial),
            "swap_free" => Some(Self::Swap_Free),
            "zero_spread" => Some(Self::Zero_Spread),
            "standard" => Some(Self::Standard),
            "stp" => Some(Self::Stp),
            "gold" => Some(Self::Gold),
            _ => None,
        }
    }
}


