
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/economic_calendar/receive.json

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


/// A list of economic events.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct EconomicCalendarResponse {
    /// Echo of the request made.
    #[serde(rename = "echo_req")]
    pub echo_req: EchoReq,
    /// Economic calendar.
    #[serde(rename = "economic_calendar", skip_serializing_if = "Option::is_none")]
    pub economic_calendar: EconomicCalendar,
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



/// Economic calendar.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct EconomicCalendar {
    /// Array of economic events
    #[serde(rename = "events", skip_serializing_if = "Option::is_none")]
    pub events: Vec<Eventsitem>,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Eventsitem {
    /// Actual value.
    #[serde(rename = "actual", skip_serializing_if = "Option::is_none")]
    pub actual: Actual,
    /// Currency symbol.
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: String,
    /// Event name.
    #[serde(rename = "event_name", skip_serializing_if = "Option::is_none")]
    pub event_name: String,
    /// Forecasted value.
    #[serde(rename = "forecast", skip_serializing_if = "Option::is_none")]
    pub forecast: Forecast,
    /// Impact.
    #[serde(rename = "impact", skip_serializing_if = "Option::is_none")]
    pub impact: i64,
    /// Previous value.
    #[serde(rename = "previous", skip_serializing_if = "Option::is_none")]
    pub previous: Previous,
    /// Release date.
    #[serde(rename = "release_date", skip_serializing_if = "Option::is_none")]
    pub release_date: i64,
}




// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Actual value.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Actual {
    /// Actual value.
    #[serde(rename = "display_value", skip_serializing_if = "Option::is_none")]
    pub display_value: String,
}






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Forecasted value.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Forecast {
    /// Forecasted value.
    #[serde(rename = "display_value", skip_serializing_if = "Option::is_none")]
    pub display_value: String,
}






// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: 

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// Previous value.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Previous {
    /// Previous value.
    #[serde(rename = "display_value", skip_serializing_if = "Option::is_none")]
    pub display_value: String,
}










