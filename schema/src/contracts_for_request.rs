
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/contracts_for/send.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// For a given symbol, get the list of currently available contracts, and the latest barrier and duration limits for each contract.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct ContractsForRequest {
    /// The short symbol name (obtained from `active_symbols` call).
    #[serde(rename = "contracts_for")]
    pub contracts_for: String,
    /// [Optional] Currency of the contract's stake and payout (obtained from `payout_currencies` call).
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: String,
    /// Deprecated - Replaced by landing_company_short.
    #[serde(rename = "landing_company", skip_serializing_if = "Option::is_none")]
    pub landing_company: LandingCompanyEnum,
    /// [Optional] Indicates which landing company to get a list of contracts for. If you are logged in, your account's landing company will override this field. Note that when landing_company_short is set to 'virtual', landing_company will take precendce until the deprecated field is removed from the api.
    #[serde(rename = "landing_company_short", skip_serializing_if = "Option::is_none")]
    pub landing_company_short: LandingCompanyShortEnum,
    /// [Optional] The login id of the user. Mandatory when multiple tokens were provided during authorize.
    #[serde(rename = "loginid", skip_serializing_if = "Option::is_none")]
    pub loginid: String,
    /// [Optional] Used to pass data through the websocket, which may be retrieved via the `echo_req` output field.
    #[serde(rename = "passthrough", skip_serializing_if = "Option::is_none")]
    pub passthrough: Passthrough,
    /// [Optional] If you specify this field, only contracts tradable through that contract type will be returned.
    #[serde(rename = "product_type", skip_serializing_if = "Option::is_none")]
    pub product_type: ProductTypeEnum,
    /// [Optional] Used to map request to response.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: i64,
}




