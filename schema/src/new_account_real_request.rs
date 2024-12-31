
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/new_account_real/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// This call opens a new real-money account. This call can be made from a virtual-money or a real-money account. If it is the latter, client information fields in this call will be ignored and data from your existing real-money account will be used.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct NewAccountRealRequest {
    /// [Optional] Purpose and reason for requesting the account opening.
    #[serde(rename = "account_opening_reason", skip_serializing_if = "Option::is_none")]
    pub account_opening_reason: AccountOpeningReasonEnum,
    /// [Optional] The anticipated account turnover.
    #[serde(rename = "account_turnover", skip_serializing_if = "Option::is_none")]
    pub account_turnover: AccountTurnoverEnum,
    /// [Optional] Within 100 characters.
    #[serde(rename = "address_city", skip_serializing_if = "Option::is_none")]
    pub address_city: String,
    /// Within 70 characters, with no leading whitespaces and may contain letters/numbers and/or any of following characters '.,:;()@#/-
    #[serde(rename = "address_line_1", skip_serializing_if = "Option::is_none")]
    pub address_line_1: String,
    /// [Optional] Within 70 characters.
    #[serde(rename = "address_line_2", skip_serializing_if = "Option::is_none")]
    pub address_line_2: String,
    /// [Optional] Within 20 characters and may not contain '+'.
    #[serde(rename = "address_postcode", skip_serializing_if = "Option::is_none")]
    pub address_postcode: String,
    /// [Optional] Possible value receive from `states_list` call.
    #[serde(rename = "address_state", skip_serializing_if = "Option::is_none")]
    pub address_state: String,
    /// [Optional] Affiliate token, within 32 characters.
    #[serde(rename = "affiliate_token", skip_serializing_if = "Option::is_none")]
    pub affiliate_token: String,
    /// [Optional] Country of legal citizenship, 2-letter country code.
    #[serde(rename = "citizen", skip_serializing_if = "Option::is_none")]
    pub citizen: Option<Value>,
    /// [Optional] Indicates whether this is for a client requesting an account with professional status.
    #[serde(rename = "client_type", skip_serializing_if = "Option::is_none")]
    pub client_type: ClientTypeEnum,
    /// [Optional] To set currency of the account. List of supported currencies can be acquired with `payout_currencies` call.
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: String,
    /// Date of birth format: `yyyy-mm-dd`.
    #[serde(rename = "date_of_birth", skip_serializing_if = "Option::is_none")]
    pub date_of_birth: String,
    /// Employment Status.
    #[serde(rename = "employment_status", skip_serializing_if = "Option::is_none")]
    pub employment_status: EmploymentStatusEnum,
    /// [Optional] Indicates client's self-declaration of FATCA.
    #[serde(rename = "fatca_declaration", skip_serializing_if = "Option::is_none")]
    pub fatca_declaration: FatcaDeclarationEnum,
    /// Within 1-50 characters, use only letters, spaces, hyphens, full-stops or apostrophes.
    #[serde(rename = "first_name", skip_serializing_if = "Option::is_none")]
    pub first_name: String,
    /// Within 1-50 characters, use only letters, spaces, hyphens, full-stops or apostrophes.
    #[serde(rename = "last_name", skip_serializing_if = "Option::is_none")]
    pub last_name: String,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// Must be `1`
    #[serde(rename = "new_account_real")]
    pub new_account_real: NewAccountRealEnum,
    /// [Optional] Indicates client's self-declaration of not being a PEP/RCA (Politically Exposed Person/Relatives and Close Associates).
    #[serde(rename = "non_pep_declaration", skip_serializing_if = "Option::is_none")]
    pub non_pep_declaration: i64,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] Starting with `+` followed by 9-35 digits, hyphens or space.
    #[serde(rename = "phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<Value>,
    /// [Optional] Place of birth, 2-letter country code.
    #[serde(rename = "place_of_birth", skip_serializing_if = "Option::is_none")]
    pub place_of_birth: String,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// 2-letter country code, possible value receive from `residence_list` call.
    #[serde(rename = "residence", skip_serializing_if = "Option::is_none")]
    pub residence: String,
    /// [Optional] Accept any value in enum list.
    #[serde(rename = "salutation", skip_serializing_if = "Option::is_none")]
    pub salutation: SalutationEnum,
    /// [Optional] Answer to secret question, within 4-50 characters. Required for new account and existing client details will be used if client open another account.
    #[serde(rename = "secret_answer", skip_serializing_if = "Option::is_none")]
    pub secret_answer: String,
    /// [Optional] Accept any value in enum list. Required for new account and existing client details will be used if client open another account.
    #[serde(rename = "secret_question", skip_serializing_if = "Option::is_none")]
    pub secret_question: SecretQuestionEnum,
    /// [Optional] Tax identification number. Only applicable for real money account. Required for `maltainvest` landing company.
    #[serde(rename = "tax_identification_number", skip_serializing_if = "Option::is_none")]
    pub tax_identification_number: String,
    /// [Optional] Residence for tax purpose. Comma separated iso country code if multiple jurisdictions. Only applicable for real money account. Required for `maltainvest` landing company.
    #[serde(rename = "tax_residence", skip_serializing_if = "Option::is_none")]
    pub tax_residence: String,
    /// [Optional] Whether the client has skipped the TIN form. Only applicable for real money account.
    #[serde(rename = "tin_skipped", skip_serializing_if = "Option::is_none")]
    pub tin_skipped: TinSkippedEnum,
    /// The tnc acceptance status of the user.
    #[serde(rename = "tnc_acceptance", skip_serializing_if = "Option::is_none")]
    pub tnc_acceptance: TncAcceptanceEnum,
}




/// [Optional] Purpose and reason for requesting the account opening.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountOpeningReasonEnum {
    Speculative,
    Income_Earning,
    Hedging,
    Peertopeer_Exchange,
}

impl AccountOpeningReasonEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Speculative => "Speculative",
            Self::Income_Earning => "Income Earning",
            Self::Hedging => "Hedging",
            Self::Peertopeer_Exchange => "Peer-to-peer exchange",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Speculative" => Some(Self::Speculative),
            "Income Earning" => Some(Self::Income_Earning),
            "Hedging" => Some(Self::Hedging),
            "Peer-to-peer exchange" => Some(Self::Peertopeer_Exchange),
            _ => None,
        }
    }
}
/// [Optional] The anticipated account turnover.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountTurnoverEnum {
    Less_Than_$25,000,
    $25,000__$50,000,
    $50,001__$100,000,
    $100,001__$500,000,
    Over_$500,000,
}

impl AccountTurnoverEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Less_Than_$25,000 => "Less than $25,000",
            Self::$25,000__$50,000 => "$25,000 - $50,000",
            Self::$50,001__$100,000 => "$50,001 - $100,000",
            Self::$100,001__$500,000 => "$100,001 - $500,000",
            Self::Over_$500,000 => "Over $500,000",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Less than $25,000" => Some(Self::Less_Than_$25,000),
            "$25,000 - $50,000" => Some(Self::$25,000__$50,000),
            "$50,001 - $100,000" => Some(Self::$50,001__$100,000),
            "$100,001 - $500,000" => Some(Self::$100,001__$500,000),
            "Over $500,000" => Some(Self::Over_$500,000),
            _ => None,
        }
    }
}
/// [Optional] Indicates whether this is for a client requesting an account with professional status.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClientTypeEnum {
    Professional,
    Retail,
}

impl ClientTypeEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Professional => "professional",
            Self::Retail => "retail",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "professional" => Some(Self::Professional),
            "retail" => Some(Self::Retail),
            _ => None,
        }
    }
}
/// [Optional] Indicates client's self-declaration of FATCA.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FatcaDeclarationEnum {
    Value0,
    Value1 = 1,
}

impl FatcaDeclarationEnum {
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
/// Must be `1`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NewAccountRealEnum {
    Value1 = 1,
}

impl NewAccountRealEnum {
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
/// [Optional] Accept any value in enum list.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SalutationEnum {
    Mr,
    Ms,
    Miss,
    Mrs,
}

impl SalutationEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Mr => "Mr",
            Self::Ms => "Ms",
            Self::Miss => "Miss",
            Self::Mrs => "Mrs",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Mr" => Some(Self::Mr),
            "Ms" => Some(Self::Ms),
            "Miss" => Some(Self::Miss),
            "Mrs" => Some(Self::Mrs),
            _ => None,
        }
    }
}
/// [Optional] Accept any value in enum list. Required for new account and existing client details will be used if client open another account.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretQuestionEnum {
    Mothers_Maiden_Name,
    Name_Of_Your_Pet,
    Name_Of_First_Love,
    Memorable_Town/city,
    Memorable_Date,
    Favourite_Dish,
    Brand_Of_First_Car,
    Favourite_Artist,
}

impl SecretQuestionEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Mothers_Maiden_Name => "Mother&#x27;s maiden name",
            Self::Name_Of_Your_Pet => "Name of your pet",
            Self::Name_Of_First_Love => "Name of first love",
            Self::Memorable_Town/city => "Memorable town/city",
            Self::Memorable_Date => "Memorable date",
            Self::Favourite_Dish => "Favourite dish",
            Self::Brand_Of_First_Car => "Brand of first car",
            Self::Favourite_Artist => "Favourite artist",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Mother&#x27;s maiden name" => Some(Self::Mothers_Maiden_Name),
            "Name of your pet" => Some(Self::Name_Of_Your_Pet),
            "Name of first love" => Some(Self::Name_Of_First_Love),
            "Memorable town/city" => Some(Self::Memorable_Town/city),
            "Memorable date" => Some(Self::Memorable_Date),
            "Favourite dish" => Some(Self::Favourite_Dish),
            "Brand of first car" => Some(Self::Brand_Of_First_Car),
            "Favourite artist" => Some(Self::Favourite_Artist),
            _ => None,
        }
    }
}
/// The tnc acceptance status of the user.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TncAcceptanceEnum {
    Value0,
    Value1 = 1,
}

impl TncAcceptanceEnum {
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
