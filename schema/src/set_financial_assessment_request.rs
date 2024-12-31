
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/set_financial_assessment/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// This call sets the financial assessment details based on the client's answers to analyze whether they possess the experience and knowledge to understand the risks involved with binary options trading.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct SetFinancialAssessmentRequest {
    /// [Optional] The anticipated account turnover.
    #[serde(rename = "account_turnover", skip_serializing_if = "Option::is_none")]
    pub account_turnover: AccountTurnoverEnum,
    /// [Optional] Binary options trading experience.
    #[serde(rename = "binary_options_trading_experience", skip_serializing_if = "Option::is_none")]
    pub binary_options_trading_experience: BinaryOptionsTradingExperienceEnum,
    /// [Optional] Binary options trading frequency.
    #[serde(rename = "binary_options_trading_frequency", skip_serializing_if = "Option::is_none")]
    pub binary_options_trading_frequency: BinaryOptionsTradingFrequencyEnum,
    /// [Optional] CFDs trading experience.
    #[serde(rename = "cfd_trading_experience", skip_serializing_if = "Option::is_none")]
    pub cfd_trading_experience: CfdTradingExperienceEnum,
    /// [Optional] CFDs trading frequency.
    #[serde(rename = "cfd_trading_frequency", skip_serializing_if = "Option::is_none")]
    pub cfd_trading_frequency: CfdTradingFrequencyEnum,
    /// [Optional] Level of Education.
    #[serde(rename = "education_level", skip_serializing_if = "Option::is_none")]
    pub education_level: EducationLevelEnum,
    /// [Optional] Industry of Employment.
    #[serde(rename = "employment_industry", skip_serializing_if = "Option::is_none")]
    pub employment_industry: EmploymentIndustryEnum,
    /// [Optional] Employment Status.
    #[serde(rename = "employment_status", skip_serializing_if = "Option::is_none")]
    pub employment_status: EmploymentStatusEnum,
    /// [Optional] Estimated Net Worth.
    #[serde(rename = "estimated_worth", skip_serializing_if = "Option::is_none")]
    pub estimated_worth: EstimatedWorthEnum,
    /// [Optional] The financial information of a client
    #[serde(rename = "financial_information", skip_serializing_if = "Option::is_none")]
    pub financial_information: FinancialInformation,
    /// [Optional] Forex trading experience.
    #[serde(rename = "forex_trading_experience", skip_serializing_if = "Option::is_none")]
    pub forex_trading_experience: ForexTradingExperienceEnum,
    /// [Optional] Forex trading frequency.
    #[serde(rename = "forex_trading_frequency", skip_serializing_if = "Option::is_none")]
    pub forex_trading_frequency: ForexTradingFrequencyEnum,
    /// [Optional] Income Source.
    #[serde(rename = "income_source", skip_serializing_if = "Option::is_none")]
    pub income_source: IncomeSourceEnum,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// [Optional] Net Annual Income.
    #[serde(rename = "net_income", skip_serializing_if = "Option::is_none")]
    pub net_income: NetIncomeEnum,
    /// [Optional] Occupation.
    #[serde(rename = "occupation", skip_serializing_if = "Option::is_none")]
    pub occupation: OccupationEnum,
    /// [Optional] Trading experience in other financial instruments.
    #[serde(rename = "other_instruments_trading_experience", skip_serializing_if = "Option::is_none")]
    pub other_instruments_trading_experience: OtherInstrumentsTradingExperienceEnum,
    /// [Optional] Trading frequency in other financial instruments.
    #[serde(rename = "other_instruments_trading_frequency", skip_serializing_if = "Option::is_none")]
    pub other_instruments_trading_frequency: OtherInstrumentsTradingFrequencyEnum,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
    /// Must be `1`
    #[serde(rename = "set_financial_assessment")]
    pub set_financial_assessment: SetFinancialAssessmentEnum,
    /// [Optional] Source of wealth.
    #[serde(rename = "source_of_wealth", skip_serializing_if = "Option::is_none")]
    pub source_of_wealth: SourceOfWealthEnum,
    /// [Optional] The trading experience of a client
    #[serde(rename = "trading_experience", skip_serializing_if = "Option::is_none")]
    pub trading_experience: TradingExperience,
    /// [Optional] The trading experience of a `maltainvest` client
    #[serde(rename = "trading_experience_regulated", skip_serializing_if = "Option::is_none")]
    pub trading_experience_regulated: TradingExperienceRegulated,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// [Optional] The financial information of a client
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct FinancialInformation {
    /// [Optional] The anticipated account turnover.
    #[serde(rename = "account_turnover", skip_serializing_if = "Option::is_none")]
    pub account_turnover: AccountTurnoverEnum,
    /// Level of Education.
    #[serde(rename = "education_level")]
    pub education_level: EducationLevelEnum,
    /// Industry of Employment.
    #[serde(rename = "employment_industry")]
    pub employment_industry: EmploymentIndustryEnum,
    /// [Optional] Employment Status.
    #[serde(rename = "employment_status", skip_serializing_if = "Option::is_none")]
    pub employment_status: EmploymentStatusEnum,
    /// Estimated Net Worth.
    #[serde(rename = "estimated_worth")]
    pub estimated_worth: EstimatedWorthEnum,
    /// Income Source.
    #[serde(rename = "income_source")]
    pub income_source: IncomeSourceEnum,
    /// Net Annual Income.
    #[serde(rename = "net_income")]
    pub net_income: NetIncomeEnum,
    /// Occupation.
    #[serde(rename = "occupation", skip_serializing_if = "Option::is_none")]
    pub occupation: OccupationEnum,
    /// [Optional] Source of wealth.
    #[serde(rename = "source_of_wealth", skip_serializing_if = "Option::is_none")]
    pub source_of_wealth: SourceOfWealthEnum,
}






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// [Optional] The trading experience of a client
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct TradingExperience {
    /// [Optional] Binary options trading experience.
    #[serde(rename = "binary_options_trading_experience", skip_serializing_if = "Option::is_none")]
    pub binary_options_trading_experience: BinaryOptionsTradingExperienceEnum,
    /// [Optional] Binary options trading frequency.
    #[serde(rename = "binary_options_trading_frequency", skip_serializing_if = "Option::is_none")]
    pub binary_options_trading_frequency: BinaryOptionsTradingFrequencyEnum,
    /// [Optional] CFDs trading experience.
    #[serde(rename = "cfd_trading_experience", skip_serializing_if = "Option::is_none")]
    pub cfd_trading_experience: CfdTradingExperienceEnum,
    /// [Optional] CFDs trading frequency.
    #[serde(rename = "cfd_trading_frequency", skip_serializing_if = "Option::is_none")]
    pub cfd_trading_frequency: CfdTradingFrequencyEnum,
    /// [Optional] Forex trading experience.
    #[serde(rename = "forex_trading_experience", skip_serializing_if = "Option::is_none")]
    pub forex_trading_experience: ForexTradingExperienceEnum,
    /// [Optional] Forex trading frequency.
    #[serde(rename = "forex_trading_frequency", skip_serializing_if = "Option::is_none")]
    pub forex_trading_frequency: ForexTradingFrequencyEnum,
    /// [Optional] Trading experience in other financial instruments.
    #[serde(rename = "other_instruments_trading_experience", skip_serializing_if = "Option::is_none")]
    pub other_instruments_trading_experience: OtherInstrumentsTradingExperienceEnum,
    /// [Optional] Trading frequency in other financial instruments.
    #[serde(rename = "other_instruments_trading_frequency", skip_serializing_if = "Option::is_none")]
    pub other_instruments_trading_frequency: OtherInstrumentsTradingFrequencyEnum,
}






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// [Optional] The trading experience of a `maltainvest` client
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct TradingExperienceRegulated {
    /// How much experience do you have in CFD trading?
    #[serde(rename = "cfd_experience")]
    pub cfd_experience: CfdExperienceEnum,
    /// How many CFD trades have you placed in the past 12 months?
    #[serde(rename = "cfd_frequency")]
    pub cfd_frequency: CfdFrequencyEnum,
    /// In your understanding, CFD trading allows you to:
    #[serde(rename = "cfd_trading_definition")]
    pub cfd_trading_definition: CfdTradingDefinitionEnum,
    /// How does leverage affect CFD trading?
    #[serde(rename = "leverage_impact_trading")]
    pub leverage_impact_trading: LeverageImpactTradingEnum,
    /// Leverage trading is high-risk, so it's a good idea to use risk management features such as stop loss. Stop loss allows you to
    #[serde(rename = "leverage_trading_high_risk_stop_loss")]
    pub leverage_trading_high_risk_stop_loss: LeverageTradingHighRiskStopLossEnum,
    /// When would you be required to pay an initial margin?
    #[serde(rename = "required_initial_margin")]
    pub required_initial_margin: RequiredInitialMarginEnum,
    /// Do you understand that you could potentially lose 100% of the money you use to trade?
    #[serde(rename = "risk_tolerance")]
    pub risk_tolerance: RiskToleranceEnum,
    /// How much knowledge and experience do you have in relation to online trading?
    #[serde(rename = "source_of_experience")]
    pub source_of_experience: SourceOfExperienceEnum,
    /// How much experience do you have with other financial instruments?
    #[serde(rename = "trading_experience_financial_instruments")]
    pub trading_experience_financial_instruments: TradingExperienceFinancialInstrumentsEnum,
    /// How many trades have you placed with other financial instruments in the past 12 months?
    #[serde(rename = "trading_frequency_financial_instruments")]
    pub trading_frequency_financial_instruments: TradingFrequencyFinancialInstrumentsEnum,
}






/// [Optional] Binary options trading experience.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BinaryOptionsTradingExperienceEnum {
    _01_Year,
    _12_Years,
    Over_3_Years,
}

impl BinaryOptionsTradingExperienceEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::_01_Year => "0-1 year",
            Self::_12_Years => "1-2 years",
            Self::Over_3_Years => "Over 3 years",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "0-1 year" => Some(Self::_01_Year),
            "1-2 years" => Some(Self::_12_Years),
            "Over 3 years" => Some(Self::Over_3_Years),
            _ => None,
        }
    }
}
/// [Optional] Binary options trading frequency.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BinaryOptionsTradingFrequencyEnum {
    _05_Transactions_In_The_Past_12_Months,
    _610_Transactions_In_The_Past_12_Months,
    _1139_Transactions_In_The_Past_12_Months,
    _40_Transactions_Or_More_In_The_Past_12_Months,
}

impl BinaryOptionsTradingFrequencyEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::_05_Transactions_In_The_Past_12_Months => "0-5 transactions in the past 12 months",
            Self::_610_Transactions_In_The_Past_12_Months => "6-10 transactions in the past 12 months",
            Self::_1139_Transactions_In_The_Past_12_Months => "11-39 transactions in the past 12 months",
            Self::_40_Transactions_Or_More_In_The_Past_12_Months => "40 transactions or more in the past 12 months",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "0-5 transactions in the past 12 months" => Some(Self::_05_Transactions_In_The_Past_12_Months),
            "6-10 transactions in the past 12 months" => Some(Self::_610_Transactions_In_The_Past_12_Months),
            "11-39 transactions in the past 12 months" => Some(Self::_1139_Transactions_In_The_Past_12_Months),
            "40 transactions or more in the past 12 months" => Some(Self::_40_Transactions_Or_More_In_The_Past_12_Months),
            _ => None,
        }
    }
}
/// [Optional] CFDs trading experience.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CfdTradingExperienceEnum {
    _01_Year,
    _12_Years,
    Over_3_Years,
}

impl CfdTradingExperienceEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::_01_Year => "0-1 year",
            Self::_12_Years => "1-2 years",
            Self::Over_3_Years => "Over 3 years",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "0-1 year" => Some(Self::_01_Year),
            "1-2 years" => Some(Self::_12_Years),
            "Over 3 years" => Some(Self::Over_3_Years),
            _ => None,
        }
    }
}
/// [Optional] CFDs trading frequency.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CfdTradingFrequencyEnum {
    _05_Transactions_In_The_Past_12_Months,
    _610_Transactions_In_The_Past_12_Months,
    _1139_Transactions_In_The_Past_12_Months,
    _40_Transactions_Or_More_In_The_Past_12_Months,
}

impl CfdTradingFrequencyEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::_05_Transactions_In_The_Past_12_Months => "0-5 transactions in the past 12 months",
            Self::_610_Transactions_In_The_Past_12_Months => "6-10 transactions in the past 12 months",
            Self::_1139_Transactions_In_The_Past_12_Months => "11-39 transactions in the past 12 months",
            Self::_40_Transactions_Or_More_In_The_Past_12_Months => "40 transactions or more in the past 12 months",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "0-5 transactions in the past 12 months" => Some(Self::_05_Transactions_In_The_Past_12_Months),
            "6-10 transactions in the past 12 months" => Some(Self::_610_Transactions_In_The_Past_12_Months),
            "11-39 transactions in the past 12 months" => Some(Self::_1139_Transactions_In_The_Past_12_Months),
            "40 transactions or more in the past 12 months" => Some(Self::_40_Transactions_Or_More_In_The_Past_12_Months),
            _ => None,
        }
    }
}
/// [Optional] Forex trading experience.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ForexTradingExperienceEnum {
    _01_Year,
    _12_Years,
    Over_3_Years,
}

impl ForexTradingExperienceEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::_01_Year => "0-1 year",
            Self::_12_Years => "1-2 years",
            Self::Over_3_Years => "Over 3 years",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "0-1 year" => Some(Self::_01_Year),
            "1-2 years" => Some(Self::_12_Years),
            "Over 3 years" => Some(Self::Over_3_Years),
            _ => None,
        }
    }
}
/// [Optional] Forex trading frequency.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ForexTradingFrequencyEnum {
    _05_Transactions_In_The_Past_12_Months,
    _610_Transactions_In_The_Past_12_Months,
    _1139_Transactions_In_The_Past_12_Months,
    _40_Transactions_Or_More_In_The_Past_12_Months,
}

impl ForexTradingFrequencyEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::_05_Transactions_In_The_Past_12_Months => "0-5 transactions in the past 12 months",
            Self::_610_Transactions_In_The_Past_12_Months => "6-10 transactions in the past 12 months",
            Self::_1139_Transactions_In_The_Past_12_Months => "11-39 transactions in the past 12 months",
            Self::_40_Transactions_Or_More_In_The_Past_12_Months => "40 transactions or more in the past 12 months",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "0-5 transactions in the past 12 months" => Some(Self::_05_Transactions_In_The_Past_12_Months),
            "6-10 transactions in the past 12 months" => Some(Self::_610_Transactions_In_The_Past_12_Months),
            "11-39 transactions in the past 12 months" => Some(Self::_1139_Transactions_In_The_Past_12_Months),
            "40 transactions or more in the past 12 months" => Some(Self::_40_Transactions_Or_More_In_The_Past_12_Months),
            _ => None,
        }
    }
}
/// [Optional] Trading experience in other financial instruments.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OtherInstrumentsTradingExperienceEnum {
    _01_Year,
    _12_Years,
    Over_3_Years,
}

impl OtherInstrumentsTradingExperienceEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::_01_Year => "0-1 year",
            Self::_12_Years => "1-2 years",
            Self::Over_3_Years => "Over 3 years",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "0-1 year" => Some(Self::_01_Year),
            "1-2 years" => Some(Self::_12_Years),
            "Over 3 years" => Some(Self::Over_3_Years),
            _ => None,
        }
    }
}
/// [Optional] Trading frequency in other financial instruments.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OtherInstrumentsTradingFrequencyEnum {
    _05_Transactions_In_The_Past_12_Months,
    _610_Transactions_In_The_Past_12_Months,
    _1139_Transactions_In_The_Past_12_Months,
    _40_Transactions_Or_More_In_The_Past_12_Months,
}

impl OtherInstrumentsTradingFrequencyEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::_05_Transactions_In_The_Past_12_Months => "0-5 transactions in the past 12 months",
            Self::_610_Transactions_In_The_Past_12_Months => "6-10 transactions in the past 12 months",
            Self::_1139_Transactions_In_The_Past_12_Months => "11-39 transactions in the past 12 months",
            Self::_40_Transactions_Or_More_In_The_Past_12_Months => "40 transactions or more in the past 12 months",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "0-5 transactions in the past 12 months" => Some(Self::_05_Transactions_In_The_Past_12_Months),
            "6-10 transactions in the past 12 months" => Some(Self::_610_Transactions_In_The_Past_12_Months),
            "11-39 transactions in the past 12 months" => Some(Self::_1139_Transactions_In_The_Past_12_Months),
            "40 transactions or more in the past 12 months" => Some(Self::_40_Transactions_Or_More_In_The_Past_12_Months),
            _ => None,
        }
    }
}
/// Must be `1`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SetFinancialAssessmentEnum {
    Value1 = 1,
}

impl SetFinancialAssessmentEnum {
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
