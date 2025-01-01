
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/new_account_maltainvest/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// This call opens a new real-money account with the `maltainvest` Landing Company. This call can be made from a virtual-money account or real-money account at Deriv (Europe) Limited. If it is the latter, client information fields in this call will be ignored and data from your existing real-money account will be used.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct NewAccountMaltainvestRequest {
    /// Show whether client has accepted risk disclaimer.
    #[serde(rename = "accept_risk", skip_serializing_if = "Option::is_none")]
    pub accept_risk: AcceptRiskEnum,
    /// [Optional] Purpose and reason for requesting the account opening.
    #[serde(rename = "account_opening_reason", skip_serializing_if = "Option::is_none")]
    pub account_opening_reason: AccountOpeningReasonEnum,
    /// [Optional] The anticipated account turnover.
    #[serde(rename = "account_turnover", skip_serializing_if = "Option::is_none")]
    pub account_turnover: AccountTurnoverEnum,
    /// Within 100 characters
    #[serde(rename = "address_city")]
    pub address_city: String,
    /// Within 70 characters, with no leading whitespaces and may contain letters/numbers and/or any of following characters '.,:;()@#/-
    #[serde(rename = "address_line_1")]
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
    /// How much experience do you have in CFD trading?
    #[serde(rename = "cfd_experience", skip_serializing_if = "Option::is_none")]
    pub cfd_experience: CfdExperienceEnum,
    /// How many CFD trades have you placed in the past 12 months?
    #[serde(rename = "cfd_frequency", skip_serializing_if = "Option::is_none")]
    pub cfd_frequency: CfdFrequencyEnum,
    /// In your understanding, CFD trading allows you to:
    #[serde(rename = "cfd_trading_definition", skip_serializing_if = "Option::is_none")]
    pub cfd_trading_definition: CfdTradingDefinitionEnum,
    /// [Optional] Country of legal citizenship, 2-letter country code. Possible value receive from `residence_list` call.
    #[serde(rename = "citizen", skip_serializing_if = "Option::is_none")]
    pub citizen: String,
    /// [Optional] Indicates whether this is for a client requesting an account with professional status.
    #[serde(rename = "client_type", skip_serializing_if = "Option::is_none")]
    pub client_type: ClientTypeEnum,
    /// [Optional] To set currency of the account. List of supported currencies can be acquired with `payout_currencies` call.
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: String,
    /// Date of birth format: yyyy-mm-dd.
    #[serde(rename = "date_of_birth")]
    pub date_of_birth: String,
    /// Level of Education
    #[serde(rename = "education_level", skip_serializing_if = "Option::is_none")]
    pub education_level: EducationLevelEnum,
    /// Industry of Employment.
    #[serde(rename = "employment_industry", skip_serializing_if = "Option::is_none")]
    pub employment_industry: EmploymentIndustryEnum,
    /// Employment Status.
    #[serde(rename = "employment_status")]
    pub employment_status: EmploymentStatusEnum,
    /// Estimated Net Worth.
    #[serde(rename = "estimated_worth", skip_serializing_if = "Option::is_none")]
    pub estimated_worth: EstimatedWorthEnum,
    /// [Optional] Indicates client's self-declaration of FATCA.
    #[serde(rename = "fatca_declaration", skip_serializing_if = "Option::is_none")]
    pub fatca_declaration: FatcaDeclarationEnum,
    /// Within 1-50 characters, use only letters, spaces, hyphens, full-stops or apostrophes.
    #[serde(rename = "first_name")]
    pub first_name: String,
    /// Income Source.
    #[serde(rename = "income_source", skip_serializing_if = "Option::is_none")]
    pub income_source: IncomeSourceEnum,
    /// Within 1-50 characters, use only letters, spaces, hyphens, full-stops or apostrophes.
    #[serde(rename = "last_name")]
    pub last_name: String,
    /// How does leverage affect CFD trading?
    #[serde(rename = "leverage_impact_trading", skip_serializing_if = "Option::is_none")]
    pub leverage_impact_trading: LeverageImpactTradingEnum,
    /// Leverage trading is high-risk, so it's a good idea to use risk management features such as stop loss. Stop loss allows you to
    #[serde(rename = "leverage_trading_high_risk_stop_loss", skip_serializing_if = "Option::is_none")]
    pub leverage_trading_high_risk_stop_loss: LeverageTradingHighRiskStopLossEnum,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// Net Annual Income.
    #[serde(rename = "net_income", skip_serializing_if = "Option::is_none")]
    pub net_income: NetIncomeEnum,
    /// Must be `1`
    #[serde(rename = "new_account_maltainvest")]
    pub new_account_maltainvest: NewAccountMaltainvestEnum,
    /// [Optional] Indicates client's self-declaration of not being a PEP/RCA.
    #[serde(rename = "non_pep_declaration", skip_serializing_if = "Option::is_none")]
    pub non_pep_declaration: i64,
    /// Occupation.
    #[serde(rename = "occupation", skip_serializing_if = "Option::is_none")]
    pub occupation: OccupationEnum,
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
    /// When would you be required to pay an initial margin?
    #[serde(rename = "required_initial_margin", skip_serializing_if = "Option::is_none")]
    pub required_initial_margin: RequiredInitialMarginEnum,
    /// 2-letter country code, possible value receive from `residence_list` call.
    #[serde(rename = "residence")]
    pub residence: String,
    /// [Optional] Indicates client's self declaration for opening account under own initiative, must be 1
    #[serde(rename = "resident_self_declaration", skip_serializing_if = "Option::is_none")]
    pub resident_self_declaration: ResidentSelfDeclarationEnum,
    /// Do you understand that you could potentially lose 100% of the money you use to trade?
    #[serde(rename = "risk_tolerance", skip_serializing_if = "Option::is_none")]
    pub risk_tolerance: RiskToleranceEnum,
    /// Accept any value in enum list.
    #[serde(rename = "salutation")]
    pub salutation: SalutationEnum,
    /// [Optional] Answer to secret question, within 4-50 characters.
    #[serde(rename = "secret_answer", skip_serializing_if = "Option::is_none")]
    pub secret_answer: String,
    /// [Optional] Accept any value in enum list.
    #[serde(rename = "secret_question", skip_serializing_if = "Option::is_none")]
    pub secret_question: SecretQuestionEnum,
    /// How much knowledge and experience do you have in relation to online trading?
    #[serde(rename = "source_of_experience", skip_serializing_if = "Option::is_none")]
    pub source_of_experience: SourceOfExperienceEnum,
    /// [Optional] Source of wealth.
    #[serde(rename = "source_of_wealth", skip_serializing_if = "Option::is_none")]
    pub source_of_wealth: SourceOfWealthEnum,
    /// Tax identification number. Only applicable for real money account. Required for `maltainvest` landing company.
    #[serde(rename = "tax_identification_number", skip_serializing_if = "Option::is_none")]
    pub tax_identification_number: String,
    /// Residence for tax purpose. Comma separated iso country code if multiple jurisdictions. Only applicable for real money account. Required for `maltainvest` landing company.
    #[serde(rename = "tax_residence")]
    pub tax_residence: String,
    /// [Optional] Whether the client has skipped the TIN form. Only applicable for real money account.
    #[serde(rename = "tin_skipped", skip_serializing_if = "Option::is_none")]
    pub tin_skipped: TinSkippedEnum,
    /// The tnc acceptance status of the user.
    #[serde(rename = "tnc_acceptance", skip_serializing_if = "Option::is_none")]
    pub tnc_acceptance: TncAcceptanceEnum,
    /// How much experience do you have with other financial instruments?
    #[serde(rename = "trading_experience_financial_instruments", skip_serializing_if = "Option::is_none")]
    pub trading_experience_financial_instruments: TradingExperienceFinancialInstrumentsEnum,
    /// How many trades have you placed with other financial instruments in the past 12 months?
    #[serde(rename = "trading_frequency_financial_instruments", skip_serializing_if = "Option::is_none")]
    pub trading_frequency_financial_instruments: TradingFrequencyFinancialInstrumentsEnum,
}




/// Show whether client has accepted risk disclaimer.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AcceptRiskEnum {
    Value0,
    Value1 = 1,
}

impl AcceptRiskEnum {
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
/// [Optional] Purpose and reason for requesting the account opening.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountOpeningReasonEnum {
    Speculative,
    Income_Earning,
    Hedging,
}

impl AccountOpeningReasonEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Speculative => "Speculative",
            Self::Income_Earning => "Income Earning",
            Self::Hedging => "Hedging",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Speculative" => Some(Self::Speculative),
            "Income Earning" => Some(Self::Income_Earning),
            "Hedging" => Some(Self::Hedging),
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
/// How much experience do you have in CFD trading?
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CfdExperienceEnum {
    No_Experience,
    Less_Than_A_Year,
    _1__2_Years,
    Over_3_Years,
}

impl CfdExperienceEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::No_Experience => "No experience",
            Self::Less_Than_A_Year => "Less than a year",
            Self::_1__2_Years => "1 - 2 years",
            Self::Over_3_Years => "Over 3 years",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "No experience" => Some(Self::No_Experience),
            "Less than a year" => Some(Self::Less_Than_A_Year),
            "1 - 2 years" => Some(Self::_1__2_Years),
            "Over 3 years" => Some(Self::Over_3_Years),
            _ => None,
        }
    }
}
/// How many CFD trades have you placed in the past 12 months?
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CfdFrequencyEnum {
    No_Transactions_In_The_Past_12_Months,
    _1__5_Transactions_In_The_Past_12_Months,
    _6__10_Transactions_In_The_Past_12_Months,
    _11__39_Transactions_In_The_Past_12_Months,
    _40_Transactions_Or_More_In_The_Past_12_Months,
}

impl CfdFrequencyEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::No_Transactions_In_The_Past_12_Months => "No transactions in the past 12 months",
            Self::_1__5_Transactions_In_The_Past_12_Months => "1 - 5 transactions in the past 12 months",
            Self::_6__10_Transactions_In_The_Past_12_Months => "6 - 10 transactions in the past 12 months",
            Self::_11__39_Transactions_In_The_Past_12_Months => "11 - 39 transactions in the past 12 months",
            Self::_40_Transactions_Or_More_In_The_Past_12_Months => "40 transactions or more in the past 12 months",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "No transactions in the past 12 months" => Some(Self::No_Transactions_In_The_Past_12_Months),
            "1 - 5 transactions in the past 12 months" => Some(Self::_1__5_Transactions_In_The_Past_12_Months),
            "6 - 10 transactions in the past 12 months" => Some(Self::_6__10_Transactions_In_The_Past_12_Months),
            "11 - 39 transactions in the past 12 months" => Some(Self::_11__39_Transactions_In_The_Past_12_Months),
            "40 transactions or more in the past 12 months" => Some(Self::_40_Transactions_Or_More_In_The_Past_12_Months),
            _ => None,
        }
    }
}
/// In your understanding, CFD trading allows you to:
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CfdTradingDefinitionEnum {
    Purchase_Shares_Of_A_Company_Or_Physical_Commodities,
    Place_A_Bet_On_The_Price_Movement,
    Speculate_On_The_Price_Movement,
    Make_A_Longterm_Investment,
}

impl CfdTradingDefinitionEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Purchase_Shares_Of_A_Company_Or_Physical_Commodities => "Purchase shares of a company or physical commodities.",
            Self::Place_A_Bet_On_The_Price_Movement => "Place a bet on the price movement.",
            Self::Speculate_On_The_Price_Movement => "Speculate on the price movement.",
            Self::Make_A_Longterm_Investment => "Make a long-term investment.",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Purchase shares of a company or physical commodities." => Some(Self::Purchase_Shares_Of_A_Company_Or_Physical_Commodities),
            "Place a bet on the price movement." => Some(Self::Place_A_Bet_On_The_Price_Movement),
            "Speculate on the price movement." => Some(Self::Speculate_On_The_Price_Movement),
            "Make a long-term investment." => Some(Self::Make_A_Longterm_Investment),
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
/// Level of Education
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EducationLevelEnum {
    Primary,
    Secondary,
    Tertiary,
}

impl EducationLevelEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Primary => "Primary",
            Self::Secondary => "Secondary",
            Self::Tertiary => "Tertiary",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Primary" => Some(Self::Primary),
            "Secondary" => Some(Self::Secondary),
            "Tertiary" => Some(Self::Tertiary),
            _ => None,
        }
    }
}
/// Industry of Employment.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmploymentIndustryEnum {
    Construction,
    Education,
    Finance,
    Health,
    Tourism,
    Information_&amp;_Communications_Technology,
    Science_&amp;_Engineering,
    Legal,
    Social_&amp;_Cultural,
    Agriculture,
    Real_Estate,
    Food_Services,
    Manufacturing,
    Unemployed,
}

impl EmploymentIndustryEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Construction => "Construction",
            Self::Education => "Education",
            Self::Finance => "Finance",
            Self::Health => "Health",
            Self::Tourism => "Tourism",
            Self::Information_&amp;_Communications_Technology => "Information &amp; Communications Technology",
            Self::Science_&amp;_Engineering => "Science &amp; Engineering",
            Self::Legal => "Legal",
            Self::Social_&amp;_Cultural => "Social &amp; Cultural",
            Self::Agriculture => "Agriculture",
            Self::Real_Estate => "Real Estate",
            Self::Food_Services => "Food Services",
            Self::Manufacturing => "Manufacturing",
            Self::Unemployed => "Unemployed",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Construction" => Some(Self::Construction),
            "Education" => Some(Self::Education),
            "Finance" => Some(Self::Finance),
            "Health" => Some(Self::Health),
            "Tourism" => Some(Self::Tourism),
            "Information &amp; Communications Technology" => Some(Self::Information_&amp;_Communications_Technology),
            "Science &amp; Engineering" => Some(Self::Science_&amp;_Engineering),
            "Legal" => Some(Self::Legal),
            "Social &amp; Cultural" => Some(Self::Social_&amp;_Cultural),
            "Agriculture" => Some(Self::Agriculture),
            "Real Estate" => Some(Self::Real_Estate),
            "Food Services" => Some(Self::Food_Services),
            "Manufacturing" => Some(Self::Manufacturing),
            "Unemployed" => Some(Self::Unemployed),
            _ => None,
        }
    }
}
/// Estimated Net Worth.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EstimatedWorthEnum {
    Less_Than_$100,000,
    $100,000__$250,000,
    $250,001__$500,000,
    $500,001__$1,000,000,
    Over_$1,000,000,
}

impl EstimatedWorthEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Less_Than_$100,000 => "Less than $100,000",
            Self::$100,000__$250,000 => "$100,000 - $250,000",
            Self::$250,001__$500,000 => "$250,001 - $500,000",
            Self::$500,001__$1,000,000 => "$500,001 - $1,000,000",
            Self::Over_$1,000,000 => "Over $1,000,000",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Less than $100,000" => Some(Self::Less_Than_$100,000),
            "$100,000 - $250,000" => Some(Self::$100,000__$250,000),
            "$250,001 - $500,000" => Some(Self::$250,001__$500,000),
            "$500,001 - $1,000,000" => Some(Self::$500,001__$1,000,000),
            "Over $1,000,000" => Some(Self::Over_$1,000,000),
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
/// Income Source.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IncomeSourceEnum {
    Salaried_Employee,
    SelfEmployed,
    Investments_&amp;_Dividends,
    Pension,
    State_Benefits,
    Savings_&amp;_Inheritance,
}

impl IncomeSourceEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Salaried_Employee => "Salaried Employee",
            Self::SelfEmployed => "Self-Employed",
            Self::Investments_&amp;_Dividends => "Investments &amp; Dividends",
            Self::Pension => "Pension",
            Self::State_Benefits => "State Benefits",
            Self::Savings_&amp;_Inheritance => "Savings &amp; Inheritance",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Salaried Employee" => Some(Self::Salaried_Employee),
            "Self-Employed" => Some(Self::SelfEmployed),
            "Investments &amp; Dividends" => Some(Self::Investments_&amp;_Dividends),
            "Pension" => Some(Self::Pension),
            "State Benefits" => Some(Self::State_Benefits),
            "Savings &amp; Inheritance" => Some(Self::Savings_&amp;_Inheritance),
            _ => None,
        }
    }
}
/// How does leverage affect CFD trading?
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LeverageImpactTradingEnum {
    Leverage_Is_A_Risk_Mitigation_Technique,
    Leverage_Prevents_You_From_Opening_Large_Positions,
    Leverage_Guarantees_Profits,
    Leverage_Lets_You_Open_Larger_Positions_For_A_Fraction_Of_The_Trades_Value,
}

impl LeverageImpactTradingEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Leverage_Is_A_Risk_Mitigation_Technique => "Leverage is a risk mitigation technique.",
            Self::Leverage_Prevents_You_From_Opening_Large_Positions => "Leverage prevents you from opening large positions.",
            Self::Leverage_Guarantees_Profits => "Leverage guarantees profits.",
            Self::Leverage_Lets_You_Open_Larger_Positions_For_A_Fraction_Of_The_Trades_Value => "Leverage lets you open larger positions for a fraction of the trade&#x27;s value.",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Leverage is a risk mitigation technique." => Some(Self::Leverage_Is_A_Risk_Mitigation_Technique),
            "Leverage prevents you from opening large positions." => Some(Self::Leverage_Prevents_You_From_Opening_Large_Positions),
            "Leverage guarantees profits." => Some(Self::Leverage_Guarantees_Profits),
            "Leverage lets you open larger positions for a fraction of the trade&#x27;s value." => Some(Self::Leverage_Lets_You_Open_Larger_Positions_For_A_Fraction_Of_The_Trades_Value),
            _ => None,
        }
    }
}
/// Leverage trading is high-risk, so it's a good idea to use risk management features such as stop loss. Stop loss allows you to
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LeverageTradingHighRiskStopLossEnum {
    Cancel_Your_Trade_At_Any_Time_Within_A_Chosen_Timeframe,
    Close_Your_Trade_Automatically_When_The_Loss_Is_More_Than_Or_Equal_To_A_Specific_Amount,
    Close_Your_Trade_Automatically_When_The_Profit_Is_More_Than_Or_Equal_To_A_Specific_Amount,
    Make_A_Guaranteed_Profit_On_Your_Trade,
}

impl LeverageTradingHighRiskStopLossEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Cancel_Your_Trade_At_Any_Time_Within_A_Chosen_Timeframe => "Cancel your trade at any time within a chosen timeframe.",
            Self::Close_Your_Trade_Automatically_When_The_Loss_Is_More_Than_Or_Equal_To_A_Specific_Amount => "Close your trade automatically when the loss is more than or equal to a specific amount.",
            Self::Close_Your_Trade_Automatically_When_The_Profit_Is_More_Than_Or_Equal_To_A_Specific_Amount => "Close your trade automatically when the profit is more than or equal to a specific amount.",
            Self::Make_A_Guaranteed_Profit_On_Your_Trade => "Make a guaranteed profit on your trade.",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Cancel your trade at any time within a chosen timeframe." => Some(Self::Cancel_Your_Trade_At_Any_Time_Within_A_Chosen_Timeframe),
            "Close your trade automatically when the loss is more than or equal to a specific amount." => Some(Self::Close_Your_Trade_Automatically_When_The_Loss_Is_More_Than_Or_Equal_To_A_Specific_Amount),
            "Close your trade automatically when the profit is more than or equal to a specific amount." => Some(Self::Close_Your_Trade_Automatically_When_The_Profit_Is_More_Than_Or_Equal_To_A_Specific_Amount),
            "Make a guaranteed profit on your trade." => Some(Self::Make_A_Guaranteed_Profit_On_Your_Trade),
            _ => None,
        }
    }
}
/// Net Annual Income.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NetIncomeEnum {
    Less_Than_$25,000,
    $25,000__$50,000,
    $50,001__$100,000,
    $100,001__$500,000,
    Over_$500,000,
}

impl NetIncomeEnum {
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
/// Must be `1`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NewAccountMaltainvestEnum {
    Value1 = 1,
}

impl NewAccountMaltainvestEnum {
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
/// Occupation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OccupationEnum {
    Chief_Executives,_Senior_Officials_And_Legislators,
    Managers,
    Professionals,
    Clerks,
    Personal_Care,_Sales_And_Service_Workers,
    Agricultural,_Forestry_And_Fishery_Workers,
    Craft,_Metal,_Electrical_And_Electronics_Workers,
    Plant_And_Machine_Operators_And_Assemblers,
    Cleaners_And_Helpers,
    Mining,_Construction,_Manufacturing_And_Transport_Workers,
    Armed_Forces,
    Government_Officers,
    Students,
    Unemployed,
}

impl OccupationEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Chief_Executives,_Senior_Officials_And_Legislators => "Chief Executives, Senior Officials and Legislators",
            Self::Managers => "Managers",
            Self::Professionals => "Professionals",
            Self::Clerks => "Clerks",
            Self::Personal_Care,_Sales_And_Service_Workers => "Personal Care, Sales and Service Workers",
            Self::Agricultural,_Forestry_And_Fishery_Workers => "Agricultural, Forestry and Fishery Workers",
            Self::Craft,_Metal,_Electrical_And_Electronics_Workers => "Craft, Metal, Electrical and Electronics Workers",
            Self::Plant_And_Machine_Operators_And_Assemblers => "Plant and Machine Operators and Assemblers",
            Self::Cleaners_And_Helpers => "Cleaners and Helpers",
            Self::Mining,_Construction,_Manufacturing_And_Transport_Workers => "Mining, Construction, Manufacturing and Transport Workers",
            Self::Armed_Forces => "Armed Forces",
            Self::Government_Officers => "Government Officers",
            Self::Students => "Students",
            Self::Unemployed => "Unemployed",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Chief Executives, Senior Officials and Legislators" => Some(Self::Chief_Executives,_Senior_Officials_And_Legislators),
            "Managers" => Some(Self::Managers),
            "Professionals" => Some(Self::Professionals),
            "Clerks" => Some(Self::Clerks),
            "Personal Care, Sales and Service Workers" => Some(Self::Personal_Care,_Sales_And_Service_Workers),
            "Agricultural, Forestry and Fishery Workers" => Some(Self::Agricultural,_Forestry_And_Fishery_Workers),
            "Craft, Metal, Electrical and Electronics Workers" => Some(Self::Craft,_Metal,_Electrical_And_Electronics_Workers),
            "Plant and Machine Operators and Assemblers" => Some(Self::Plant_And_Machine_Operators_And_Assemblers),
            "Cleaners and Helpers" => Some(Self::Cleaners_And_Helpers),
            "Mining, Construction, Manufacturing and Transport Workers" => Some(Self::Mining,_Construction,_Manufacturing_And_Transport_Workers),
            "Armed Forces" => Some(Self::Armed_Forces),
            "Government Officers" => Some(Self::Government_Officers),
            "Students" => Some(Self::Students),
            "Unemployed" => Some(Self::Unemployed),
            _ => None,
        }
    }
}
/// When would you be required to pay an initial margin?
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequiredInitialMarginEnum {
    When_Opening_A_Leveraged_CFD_Trade,
    When_Trading_Multipliers,
    When_Buying_Shares_Of_A_Company,
    All_Of_The_Above,
}

impl RequiredInitialMarginEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::When_Opening_A_Leveraged_CFD_Trade => "When opening a Leveraged CFD trade.",
            Self::When_Trading_Multipliers => "When trading Multipliers.",
            Self::When_Buying_Shares_Of_A_Company => "When buying shares of a company.",
            Self::All_Of_The_Above => "All of the above.",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "When opening a Leveraged CFD trade." => Some(Self::When_Opening_A_Leveraged_CFD_Trade),
            "When trading Multipliers." => Some(Self::When_Trading_Multipliers),
            "When buying shares of a company." => Some(Self::When_Buying_Shares_Of_A_Company),
            "All of the above." => Some(Self::All_Of_The_Above),
            _ => None,
        }
    }
}
/// [Optional] Indicates client's self declaration for opening account under own initiative, must be 1
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResidentSelfDeclarationEnum {
    Value1 = 1,
}

impl ResidentSelfDeclarationEnum {
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
/// Do you understand that you could potentially lose 100% of the money you use to trade?
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RiskToleranceEnum {
    Yes,
    No,
}

impl RiskToleranceEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Yes => "Yes",
            Self::No => "No",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Yes" => Some(Self::Yes),
            "No" => Some(Self::No),
            _ => None,
        }
    }
}
/// Accept any value in enum list.
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
/// [Optional] Accept any value in enum list.
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
/// How much knowledge and experience do you have in relation to online trading?
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceOfExperienceEnum {
    I_Have_An_Academic_Degree,_Professional_Certification,_And/or_Work_Experience,
    I_Trade_Forex_CFDs_And_Other_Complex_Financial_Instruments,
    I_Have_Attended_Seminars,_Training,_And/or_Workshops,
    I_Have_Little_Experience,
    I_Have_No_Knowledge,
}

impl SourceOfExperienceEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::I_Have_An_Academic_Degree,_Professional_Certification,_And/or_Work_Experience => "I have an academic degree, professional certification, and/or work experience.",
            Self::I_Trade_Forex_CFDs_And_Other_Complex_Financial_Instruments => "I trade forex CFDs and other complex financial instruments.",
            Self::I_Have_Attended_Seminars,_Training,_And/or_Workshops => "I have attended seminars, training, and/or workshops.",
            Self::I_Have_Little_Experience => "I have little experience.",
            Self::I_Have_No_Knowledge => "I have no knowledge.",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "I have an academic degree, professional certification, and/or work experience." => Some(Self::I_Have_An_Academic_Degree,_Professional_Certification,_And/or_Work_Experience),
            "I trade forex CFDs and other complex financial instruments." => Some(Self::I_Trade_Forex_CFDs_And_Other_Complex_Financial_Instruments),
            "I have attended seminars, training, and/or workshops." => Some(Self::I_Have_Attended_Seminars,_Training,_And/or_Workshops),
            "I have little experience." => Some(Self::I_Have_Little_Experience),
            "I have no knowledge." => Some(Self::I_Have_No_Knowledge),
            _ => None,
        }
    }
}
/// [Optional] Source of wealth.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceOfWealthEnum {
    Accumulation_Of_Income/Savings,
    Cash_Business,
    Company_Ownership,
    Divorce_Settlement,
    Inheritance,
    Investment_Income,
    Sale_Of_Property,
}

impl SourceOfWealthEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Accumulation_Of_Income/Savings => "Accumulation of Income/Savings",
            Self::Cash_Business => "Cash Business",
            Self::Company_Ownership => "Company Ownership",
            Self::Divorce_Settlement => "Divorce Settlement",
            Self::Inheritance => "Inheritance",
            Self::Investment_Income => "Investment Income",
            Self::Sale_Of_Property => "Sale of Property",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Accumulation of Income/Savings" => Some(Self::Accumulation_Of_Income/Savings),
            "Cash Business" => Some(Self::Cash_Business),
            "Company Ownership" => Some(Self::Company_Ownership),
            "Divorce Settlement" => Some(Self::Divorce_Settlement),
            "Inheritance" => Some(Self::Inheritance),
            "Investment Income" => Some(Self::Investment_Income),
            "Sale of Property" => Some(Self::Sale_Of_Property),
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
/// How much experience do you have with other financial instruments?
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TradingExperienceFinancialInstrumentsEnum {
    No_Experience,
    Less_Than_A_Year,
    _1__2_Years,
    Over_3_Years,
}

impl TradingExperienceFinancialInstrumentsEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::No_Experience => "No experience",
            Self::Less_Than_A_Year => "Less than a year",
            Self::_1__2_Years => "1 - 2 years",
            Self::Over_3_Years => "Over 3 years",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "No experience" => Some(Self::No_Experience),
            "Less than a year" => Some(Self::Less_Than_A_Year),
            "1 - 2 years" => Some(Self::_1__2_Years),
            "Over 3 years" => Some(Self::Over_3_Years),
            _ => None,
        }
    }
}
/// How many trades have you placed with other financial instruments in the past 12 months?
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TradingFrequencyFinancialInstrumentsEnum {
    No_Transactions_In_The_Past_12_Months,
    _1__5_Transactions_In_The_Past_12_Months,
    _6__10_Transactions_In_The_Past_12_Months,
    _11__39_Transactions_In_The_Past_12_Months,
    _40_Transactions_Or_More_In_The_Past_12_Months,
}

impl TradingFrequencyFinancialInstrumentsEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::No_Transactions_In_The_Past_12_Months => "No transactions in the past 12 months",
            Self::_1__5_Transactions_In_The_Past_12_Months => "1 - 5 transactions in the past 12 months",
            Self::_6__10_Transactions_In_The_Past_12_Months => "6 - 10 transactions in the past 12 months",
            Self::_11__39_Transactions_In_The_Past_12_Months => "11 - 39 transactions in the past 12 months",
            Self::_40_Transactions_Or_More_In_The_Past_12_Months => "40 transactions or more in the past 12 months",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "No transactions in the past 12 months" => Some(Self::No_Transactions_In_The_Past_12_Months),
            "1 - 5 transactions in the past 12 months" => Some(Self::_1__5_Transactions_In_The_Past_12_Months),
            "6 - 10 transactions in the past 12 months" => Some(Self::_6__10_Transactions_In_The_Past_12_Months),
            "11 - 39 transactions in the past 12 months" => Some(Self::_11__39_Transactions_In_The_Past_12_Months),
            "40 transactions or more in the past 12 months" => Some(Self::_40_Transactions_Or_More_In_The_Past_12_Months),
            _ => None,
        }
    }
}
