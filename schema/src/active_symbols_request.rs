
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/active_symbols/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Retrieve a list of all currently active symbols (underlying markets upon which contracts are available for trading).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct ActiveSymbolsRequest {
    /// If you use `brief`, only a subset of fields will be returned.
    #[serde(rename = "active_symbols")]
    pub active_symbols: ActiveSymbolsEnum,
    /// [Optional] Category of barrier.
    #[serde(rename = "barrier_category", skip_serializing_if = "Option::is_none")]
    pub barrier_category: Vec<BarrierCategoryitemEnum>,
    /// [Optional] The proposed contract type
    #[serde(rename = "contract_type", skip_serializing_if = "Option::is_none")]
    pub contract_type: Vec<ContractTypeitemEnum>,
    /// Deprecated - replaced by landing_company_short.
    #[serde(rename = "landing_company", skip_serializing_if = "Option::is_none")]
    pub landing_company: LandingCompanyEnum,
    /// [Optional] If you specify this field, only symbols available for trading by that landing company will be returned. If you are logged in, only symbols available for trading by your landing company will be returned regardless of what you specify in this field.
    #[serde(rename = "landing_company_short", skip_serializing_if = "Option::is_none")]
    pub landing_company_short: LandingCompanyShortEnum,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] If you specify this field, only symbols that can be traded through that product type will be returned.
    #[serde(rename = "product_type", skip_serializing_if = "Option::is_none")]
    pub product_type: ProductTypeEnum,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
}




/// If you use `brief`, only a subset of fields will be returned.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActiveSymbolsEnum {
    Brief,
    Full,
}

impl ActiveSymbolsEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Brief => "brief",
            Self::Full => "full",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "brief" => Some(Self::Brief),
            "full" => Some(Self::Full),
            _ => None,
        }
    }
}
/// 
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BarrierCategoryitemEnum {
    American,
    Asian,
    Euro_Atm,
    Euro_Non_Atm,
    Non_Financial,
    Lookback,
    Reset,
}

impl BarrierCategoryitemEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::American => "american",
            Self::Asian => "asian",
            Self::Euro_Atm => "euro_atm",
            Self::Euro_Non_Atm => "euro_non_atm",
            Self::Non_Financial => "non_financial",
            Self::Lookback => "lookback",
            Self::Reset => "reset",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "american" => Some(Self::American),
            "asian" => Some(Self::Asian),
            "euro_atm" => Some(Self::Euro_Atm),
            "euro_non_atm" => Some(Self::Euro_Non_Atm),
            "non_financial" => Some(Self::Non_Financial),
            "lookback" => Some(Self::Lookback),
            "reset" => Some(Self::Reset),
            _ => None,
        }
    }
}
/// 
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContractTypeitemEnum {
    MULTUP,
    MULTDOWN,
    UPORDOWN,
    EXPIRYRANGE,
    ONETOUCH,
    CALLE,
    LBHIGHLOW,
    ASIAND,
    EXPIRYRANGEE,
    DIGITDIFF,
    DIGITMATCH,
    DIGITOVER,
    PUTE,
    DIGITUNDER,
    NOTOUCH,
    CALL,
    RANGE,
    LBFLOATPUT,
    DIGITODD,
    PUT,
    ASIANU,
    LBFLOATCALL,
    EXPIRYMISSE,
    EXPIRYMISS,
    DIGITEVEN,
    TICKHIGH,
    TICKLOW,
    RESETCALL,
    RESETPUT,
    CALLSPREAD,
    PUTSPREAD,
    RUNHIGH,
    RUNLOW,
    ACCU,
    VANILLALONGCALL,
    VANILLALONGPUT,
    TURBOSLONG,
    TURBOSSHORT,
}

impl ContractTypeitemEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MULTUP => "MULTUP",
            Self::MULTDOWN => "MULTDOWN",
            Self::UPORDOWN => "UPORDOWN",
            Self::EXPIRYRANGE => "EXPIRYRANGE",
            Self::ONETOUCH => "ONETOUCH",
            Self::CALLE => "CALLE",
            Self::LBHIGHLOW => "LBHIGHLOW",
            Self::ASIAND => "ASIAND",
            Self::EXPIRYRANGEE => "EXPIRYRANGEE",
            Self::DIGITDIFF => "DIGITDIFF",
            Self::DIGITMATCH => "DIGITMATCH",
            Self::DIGITOVER => "DIGITOVER",
            Self::PUTE => "PUTE",
            Self::DIGITUNDER => "DIGITUNDER",
            Self::NOTOUCH => "NOTOUCH",
            Self::CALL => "CALL",
            Self::RANGE => "RANGE",
            Self::LBFLOATPUT => "LBFLOATPUT",
            Self::DIGITODD => "DIGITODD",
            Self::PUT => "PUT",
            Self::ASIANU => "ASIANU",
            Self::LBFLOATCALL => "LBFLOATCALL",
            Self::EXPIRYMISSE => "EXPIRYMISSE",
            Self::EXPIRYMISS => "EXPIRYMISS",
            Self::DIGITEVEN => "DIGITEVEN",
            Self::TICKHIGH => "TICKHIGH",
            Self::TICKLOW => "TICKLOW",
            Self::RESETCALL => "RESETCALL",
            Self::RESETPUT => "RESETPUT",
            Self::CALLSPREAD => "CALLSPREAD",
            Self::PUTSPREAD => "PUTSPREAD",
            Self::RUNHIGH => "RUNHIGH",
            Self::RUNLOW => "RUNLOW",
            Self::ACCU => "ACCU",
            Self::VANILLALONGCALL => "VANILLALONGCALL",
            Self::VANILLALONGPUT => "VANILLALONGPUT",
            Self::TURBOSLONG => "TURBOSLONG",
            Self::TURBOSSHORT => "TURBOSSHORT",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "MULTUP" => Some(Self::MULTUP),
            "MULTDOWN" => Some(Self::MULTDOWN),
            "UPORDOWN" => Some(Self::UPORDOWN),
            "EXPIRYRANGE" => Some(Self::EXPIRYRANGE),
            "ONETOUCH" => Some(Self::ONETOUCH),
            "CALLE" => Some(Self::CALLE),
            "LBHIGHLOW" => Some(Self::LBHIGHLOW),
            "ASIAND" => Some(Self::ASIAND),
            "EXPIRYRANGEE" => Some(Self::EXPIRYRANGEE),
            "DIGITDIFF" => Some(Self::DIGITDIFF),
            "DIGITMATCH" => Some(Self::DIGITMATCH),
            "DIGITOVER" => Some(Self::DIGITOVER),
            "PUTE" => Some(Self::PUTE),
            "DIGITUNDER" => Some(Self::DIGITUNDER),
            "NOTOUCH" => Some(Self::NOTOUCH),
            "CALL" => Some(Self::CALL),
            "RANGE" => Some(Self::RANGE),
            "LBFLOATPUT" => Some(Self::LBFLOATPUT),
            "DIGITODD" => Some(Self::DIGITODD),
            "PUT" => Some(Self::PUT),
            "ASIANU" => Some(Self::ASIANU),
            "LBFLOATCALL" => Some(Self::LBFLOATCALL),
            "EXPIRYMISSE" => Some(Self::EXPIRYMISSE),
            "EXPIRYMISS" => Some(Self::EXPIRYMISS),
            "DIGITEVEN" => Some(Self::DIGITEVEN),
            "TICKHIGH" => Some(Self::TICKHIGH),
            "TICKLOW" => Some(Self::TICKLOW),
            "RESETCALL" => Some(Self::RESETCALL),
            "RESETPUT" => Some(Self::RESETPUT),
            "CALLSPREAD" => Some(Self::CALLSPREAD),
            "PUTSPREAD" => Some(Self::PUTSPREAD),
            "RUNHIGH" => Some(Self::RUNHIGH),
            "RUNLOW" => Some(Self::RUNLOW),
            "ACCU" => Some(Self::ACCU),
            "VANILLALONGCALL" => Some(Self::VANILLALONGCALL),
            "VANILLALONGPUT" => Some(Self::VANILLALONGPUT),
            "TURBOSLONG" => Some(Self::TURBOSLONG),
            "TURBOSSHORT" => Some(Self::TURBOSSHORT),
            _ => None,
        }
    }
}
