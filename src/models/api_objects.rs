use serde::{Deserialize, Serialize};
use serde_json::Value;


/// ErrorResponse is the error struct that is being returned by this library
///
/// It has THREE major components
///
/// error: block contains errors from razorpay directly parsed in.
///
/// lib_error_code: contains library specific code to check for parsing ad other internal errors.
///     These errors are not related to the actual razorpay errors. For Razorpay errors use the error block
///
/// lib_error_description: This is the description of the library error code. The description may change time to time
///     so if checking for a particular error, use the lib_error_code as those are very unlikely to change.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    /// Razorpay Error Object
    pub error: RzpErrorObject,
    /// library error code
    pub lib_error_code: String,
    /// library error description
    pub lib_error_description: String,
    pub lib_error: Option(String),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RzpErrorObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    #[serde(rename = "payment_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
    #[serde(rename = "order_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
}