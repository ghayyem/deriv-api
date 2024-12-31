
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/payment_methods/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// List of available payment methods for a given country.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct PaymentMethodsResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Available payment methods for a given country. Note: if a user is logged in, the residence country will be considered.
    #[serde(rename = "payment_methods", skip_serializing_if = "Option::is_none")]
    pub payment_methods: Vec<PaymentMethodsitem>,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// A payment method suported for the given country
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct PaymentMethodsitem {
    /// The min and max values for deposits.
    #[serde(rename = "deposit_limits", flatten)]
    pub deposit_limits: HashMap<String, DepositLimitsvalue>,
    /// How much time it takes for a deposit to be processed.
    #[serde(rename = "deposit_time")]
    pub deposit_time: String,
    /// Short description explaining the payment method.
    #[serde(rename = "description")]
    pub description: String,
    /// Common name for the payment method.
    #[serde(rename = "display_name")]
    pub display_name: String,
    /// Unique identifier for the payment method.
    #[serde(rename = "id")]
    pub id: String,
    /// Payment processor for this payment method.
    #[serde(rename = "payment_processor")]
    pub payment_processor: String,
    /// A list of predefined amounts for withdraw or deposit.
    #[serde(rename = "predefined_amounts")]
    pub predefined_amounts: Vec<i64>,
    /// Sign up link for this payment method.
    #[serde(rename = "signup_link")]
    pub signup_link: String,
    /// Currencies supported for this payment method.
    #[serde(rename = "supported_currencies")]
    pub supported_currencies: Vec<String>,
    /// Type of Payment Method.
    #[serde(rename = "type")]
    pub type: String,
    /// A printable description for type of payment method.
    #[serde(rename = "type_display_name")]
    pub type_display_name: String,
    /// Withdrawal limits per currency.
    #[serde(rename = "withdraw_limits", flatten)]
    pub withdraw_limits: HashMap<String, WithdrawLimitsvalue>,
    /// How much time takes a withdrawal to be processed.
    #[serde(rename = "withdrawal_time")]
    pub withdrawal_time: String,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Deposit limits for this method.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct DepositLimitsvalue {
    /// Maximum amount for deposits for this currency.
    #[serde(rename = "max")]
    pub max: i64,
    /// Minimum amount for deposit for this currency.
    #[serde(rename = "min")]
    pub min: i64,
}






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Withdrawal limits for this currency.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct WithdrawLimitsvalue {
    /// Maximum amount for wihdrawals in this currency.
    #[serde(rename = "max")]
    pub max: i64,
    /// Minimum amount for withdrawals in this currency.
    #[serde(rename = "min")]
    pub min: i64,
}








