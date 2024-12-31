
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/p2p_country_list/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// List all or specific country and its payment methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct P2pCountryListResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Action name of the request made.
    #[serde(rename = "msg_type")]
    pub msg_type: MsgTypeEnum,
    /// Country identified by country code
    #[serde(rename = "p2p_country_list", skip_serializing_if = "Option::is_none", flatten)]
    pub p_2p_country_list: HashMap<String, P2pCountryListvalue>,
    /// Optional field sent in request to map to response, present only when request contains `req_id`.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Country code identifier.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct P2pCountryListvalue {
    /// Display name of country.
    #[serde(rename = "country_name")]
    pub country_name: String,
    /// When 1, users in this country may place orders on ads in other countries.
    #[serde(rename = "cross_border_ads_enabled")]
    pub cross_border_ads_enabled: CrossBorderAdsEnabledEnum,
    /// Availability of fixed rate adverts.
    #[serde(rename = "fixed_rate_adverts")]
    pub fixed_rate_adverts: FixedRateAdvertsEnum,
    /// Availability of floating rate adverts.
    #[serde(rename = "float_rate_adverts")]
    pub float_rate_adverts: FloatRateAdvertsEnum,
    /// Maximum rate offset for floating rate adverts.
    #[serde(rename = "float_rate_offset_limit")]
    pub float_rate_offset_limit: f64,
    /// Local currency of the country.
    #[serde(rename = "local_currency")]
    pub local_currency: String,
    /// Payment method identifier.
    #[serde(rename = "payment_methods", flatten)]
    pub payment_methods: HashMap<String, PaymentMethodsvalue>,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// Payment method identifier.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct PaymentMethodsvalue {
    /// Display name of payment method.
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: String,
    /// Payment method field definitions.
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none", flatten)]
    pub fields: HashMap<String, Fieldsvalue>,
    /// Payment method type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type: TypeEnum,
}








