// bitcoin_de_trading_api_sdk_v4/errors
use reqwest::StatusCode;
use serde::Deserialize;
use thiserror::Error;
use std::time::SystemTimeError;
use reqwest::header::InvalidHeaderValue;


/// Represents a single error detail within the API response.
/// Based on the "Error-Details" table in the documentation.
#[derive(Debug, Deserialize)]
pub struct ApiErrorDetail {
    pub message: String,
    // The documentation says 'code' is string, but example is integer (50).
    // The constants are i32. Let's use i32, which aligns with constants and example value.
    pub code: i32,
    pub field: Option<String>, // Optional field name
}


/// Represents the structure of an error response from the Bitcoin.de API.
///
/// This struct is used to deserialize the JSON body returned by the API
/// when an error occurs.
#[derive(Debug, Deserialize)]
pub struct ApiErrorBody {
    /// A list of error codes returned by the API.
    pub errors: Vec<i32>,
    /// A list of error messages corresponding to the error codes.
    pub messages: Vec<String>,
}

/// Custom error type for the Bitcoin.de Trading API SDK.
///
/// This enum encapsulates all possible errors that can occur when using the SDK,
/// including network errors, serialization issues, API-specific errors, and
/// SDK internal errors.
#[derive(Error, Debug)]
pub enum Error {
    /// An error occurred while making an HTTP request using `reqwest`.
    #[error("HTTP request failed: {0}")]
    Reqwest(#[from] reqwest::Error),

    /// An error occurred during URL parsing or construction.
    #[error("URL parsing error: {0}")]
    Url(#[from] url::ParseError),

    /// An error occurred during URL encoding/decoding of parameters.
    #[error("URL encoding error: {0}")]
    UrlEncoded(#[from] serde_urlencoded::ser::Error),

    /// An error occurred during JSON serialization or deserialization.
    #[error("JSON serialization/deserialization error: {0}")]
    Json(#[from] serde_json::Error),

    /// An error occurred during HMAC key initialization.
    #[error("HMAC key initialization error: {0}")]
    HmacKey(#[from] hmac::digest::InvalidLength),

    /// An error occurred getting the system time or calculating duration.
    #[error("System time error: {0}")]
    SystemTime(#[from] SystemTimeError),

    /// An error occurred due to an invalid HTTP header value.
    #[error("Invalid HTTP header value: {0}")]
    InvalidHeaderValue(#[from] InvalidHeaderValue),
    /// An error occurred because a required API method was not found in settings.
    #[error("API method '{0}' not found in method settings")]
    MethodNotFound(&'static str),

    /// An error occurred because a required path parameter was missing.
    #[error("Missing required path parameter: {0}")]
    MissingPathParameter(&'static str),

    /// An error occurred because the API request returned a non-success status code.
    #[error("API request failed with status {status}: {body}")]
    Api {
        status: StatusCode,
        body: String, // Keep the raw body for inspection
        api_error_details: Option<ApiErrorBody>, // Optional structured error details
    },

    /// A generic error for cases not covered by specific variants.
    #[error("An unknown error occurred: {0}")]
    Other(String),
}

impl Error {
    /// Creates an `Api` error variant, attempting to parse the body as JSON.
    pub(crate) fn api_error(status: StatusCode, body: String) -> Self {
        let api_error_details: Option<ApiErrorBody> = serde_json::from_str(&body).ok();
        Error::Api {
            status,
            body,
            api_error_details,
        }
    }
}