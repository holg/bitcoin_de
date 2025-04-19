use std::sync::Arc;
use axum::{extract::State, http::StatusCode, response::Json, extract::Path};
// Import SDK components needed by handlers
use bitcoin_de::bitcoin_de_trading_api_sdk_v4::TradingApiSdkV4; // The SDK client struct
// use bitcoin_de::bitcoin_de_trading_api_sdk_v4::errors::Error; // SDK Error type
// TODO implement IntoResponse trait for custom error responses and use it here 
// Import enums and response structs used by handlers
use bitcoin_de::enums::TradingPair; // Import the TradingPair enum (needed for from_str)
use bitcoin_de::bitcoin_de_trading_api_sdk_v4::responses::account::ShowAccountInfoResponse; // Response struct for account info
use bitcoin_de::bitcoin_de_trading_api_sdk_v4::responses::misc::ShowRatesResponse; // Response struct for rates
// Import the error detail struct for potential error responses
use bitcoin_de::bitcoin_de_trading_api_sdk_v4::errors::ApiErrorDetail; // Assuming ApiErrorDetail is defined and public


/// Handles requests to retrieve account information from the Bitcoin.de API.
///
/// This handler calls the SDK's `show_account_info` method to fetch the current
/// user's account details, including balance information, trading fees, and other
/// account-related data.
///
/// # Parameters
/// * `sdk` - The shared Bitcoin.de Trading API SDK client instance, extracted from
///   the application state. This client is used to make the authenticated API call.
///
/// # Returns
/// * `Result<Json<ShowAccountInfoResponse>, StatusCode>` - On success, returns the account
///   information wrapped in a JSON response with a 200 OK status. On failure, returns
///   an appropriate HTTP status code (currently a generic 500 Internal Server Error).
#[axum::debug_handler]
pub async fn handle_show_account_info(
    // State extractor to get the shared SDK client instance
    State(sdk): State<Arc<TradingApiSdkV4>>,
) -> Result<Json<ShowAccountInfoResponse>, StatusCode> { // Return JSON on success, HTTP status on error
    // Call the asynchronous SDK method
    let result = sdk.show_account_info().await;

    // Map the SDK's Result to Axum's Result
    match result {
        Ok(response) => {
            // On success, wrap the response struct in Json and return OK (200)
            Ok(Json(response))
        }
        Err(e) => {
            // On error, convert the SDK Error to an appropriate HTTP status code.
            // For now, we'll log the detailed error and return a generic 500 Internal Server Error.
            // TODO: Implement more specific error mapping from SDK Error variants
            // to different HTTP status codes (e.g., 400 for bad request errors,
            // 401/403 for auth errors, 404 for not found, 429 for rate limits).
            eprintln!("Error in showAccountInfo handler: {}", e); // Log the detailed error

            // Return a generic InternalServerError for now
            Err(StatusCode::INTERNAL_SERVER_ERROR)
            // Example of more specific error mapping (requires matching on Error variants):
            // match e {
            //     Error::Api { status, .. } => Err(status), // If SDK Error includes the original HTTP status
            //     Error::MissingPathParameter(_) | Error::MethodNotFound(_) => Err(StatusCode::BAD_REQUEST),
            //     _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
            // }
        }
    }
}

/// Handles requests to retrieve current exchange rates from the Bitcoin.de API.
///
/// This handler processes requests to the `/api/v4/rates/:trading_pair` endpoint,
/// fetching the current exchange rates for the specified trading pair. It converts
/// the string trading pair parameter to the appropriate enum value before making
/// the API call.
///
/// # Parameters
/// * `sdk` - The shared Bitcoin.de Trading API SDK client instance, extracted from
///   the application state. This client is used to make the authenticated API call.
/// * `trading_pair_str` - A string representing the trading pair (e.g., "btceur",
///   "etheur") extracted from the URL path. Must be convertible to a valid
///   `TradingPair` enum variant.
///
/// # Returns
/// * `Result<Json<ShowRatesResponse>, StatusCode>` - On success, returns the exchange
///   rate information wrapped in a JSON response with a 200 OK status. On failure,
///   returns an appropriate HTTP status code (400 for invalid trading pair, or 500
///   for other errors).
#[axum::debug_handler]
pub async fn handle_show_rates(
    State(sdk): State<Arc<TradingApiSdkV4>>,
    Path(trading_pair_str): Path<String>,
) -> Result<Json<ShowRatesResponse>, StatusCode> {
    let trading_pair = match TradingPair::from_str(&trading_pair_str) {
        Ok(pair) => pair,
        Err(_) => {
            eprintln!("Invalid trading pair received in handler: {}", trading_pair_str);
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    let result = sdk.show_rates(trading_pair).await;

    match result {
        Ok(response) => {
            Ok(Json(response))
        }
        Err(e) => {
            eprintln!("Error in showRates handler for {}: {}", trading_pair_str, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}