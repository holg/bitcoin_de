// bitcoin_de_trading_api_sdk_v4/trading_api_sdk_v4.rs
use hmac::{Hmac, Mac};
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use sha2::Sha256;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Arc;
use url::Url;
use serde::de::DeserializeOwned;
use serde_json::from_str;
use serde_urlencoded;

use crate::bitcoin_de_trading_api_sdk_v4::config::ApiCredentials;
use crate::bitcoin_de_trading_api_sdk_v4::constants::API_BASE_URI;
use crate::bitcoin_de_trading_api_sdk_v4::method_settings::{MethodSetting, METHOD_SETTINGS};
use crate::bitcoin_de_trading_api_sdk_v4::errors::Error;
use crate::bitcoin_de_trading_api_sdk_v4::enums::TradingPair;
// Use the relative path for the responses module from within this file
use crate::bitcoin_de_trading_api_sdk_v4::responses::*;

// Import constants for method names and parameter names
use crate::bitcoin_de_trading_api_sdk_v4::method_settings::constants::*;


/// Represents a client for interacting with the Bitcoin.de Trading API v4.
///
/// This struct encapsulates the necessary components for making authenticated
/// requests to the Bitcoin.de API, including credentials, method configurations,
/// and an HTTP client for executing requests.
///
/// # Fields
///
/// * `api_credentials` - The API key and secret used for authentication with the Bitcoin.de API.
/// * `method_settings` - A static reference to a map of API method configurations, defining
///   endpoints and their requirements.
/// * `client` - A reusable HTTP client for making requests to the API.
pub struct TradingApiSdkV4 {
    api_credentials: ApiCredentials,
    method_settings: &'static HashMap<&'static str, MethodSetting>,
    client: Arc<Client>, // Re-use the client
}

impl TradingApiSdkV4 {
    /// Creates a new instance of the Bitcoin.de Trading API SDK v4 client.
    ///
    /// This constructor initializes a new API client with the provided credentials
    /// and sets up the HTTP client for making requests to the Bitcoin.de API.
    ///
    /// # Parameters
    ///
    /// * `api_key` - The API key for authenticating with the Bitcoin.de API.
    /// * `api_secret` - The API secret used for signing API requests to the Bitcoin.de API.
    ///
    /// # Returns
    ///
    /// A new instance of `TradingApiSdkV4` configured with the provided credentials.
    ///
    /// # Example
    ///
    /// ```
    /// use bitcoin_de::bitcoin_de_trading_api_sdk_v4::TradingApiSdkV4;
    ///
    /// let client = TradingApiSdkV4::new("your_api_key".to_string(), "your_api_secret".to_string());
    /// ```
    pub fn new(api_key: String, api_secret: String) -> Self {
        TradingApiSdkV4 {
            api_credentials: ApiCredentials { api_key, api_secret },
            method_settings: &METHOD_SETTINGS,
            client: Client::new().into(), // Create client once
        }
    }

    /// Constructs a complete URL path by replacing placeholders in the method's path segments with actual values.
    ///
    /// This function processes each segment of the API endpoint path, replacing placeholders (segments
    /// starting with ':') with corresponding values from the provided parameters map. It then combines
    /// these processed segments with the API base URI to form a complete URL path.
    ///
    /// # Parameters
    ///
    /// * `method_setting` - The method configuration containing path segments and other API endpoint details.
    /// * `path_params` - A mutable map of parameter names to values. Parameters used in the path are removed
    ///   from this map during processing to track which ones have been consumed.
    ///
    /// # Returns
    ///
    /// * `Result<String, Error>` - On success, returns the complete URL path with all placeholders
    ///   replaced. On failure (e.g., missing required path parameter), returns an error.
    fn build_url_path(
        method_setting: &MethodSetting,
        path_params: &mut HashMap<&'static str, String>, // Mutable to remove used params
    ) -> Result<String, Error> {
        let mut processed_segments: Vec<String> = Vec::new();
        // Use the defined path_segments from the method setting
        for segment in method_setting.path_segments {
            if segment.starts_with(':') {
                let param_name = &segment[1..];
                if let Some(value) = path_params.remove(param_name) {
                    // TODO: Consider URL-encoding path segments if they can contain special characters
                    processed_segments.push(value); // Use the provided value
                } else {
                    // If the placeholder wasn't found in params, it's an error
                    return Err(Error::MissingPathParameter(param_name));
                }
            } else {
                // It's a literal segment
                processed_segments.push(segment.to_string());
            }
        }

        // Ensure base URI doesn't end with '/' before joining
        let base = API_BASE_URI.trim_end_matches('/');
        // Join the base and the processed segments
        let final_path = format!("{}/{}", base, processed_segments.join("/"));

        Ok(final_path)
    }

    /// Executes an API request to the Bitcoin.de Trading API.
    ///
    /// This internal method handles the core request building, signing, and sending.
    /// Public API methods should wrap calls to this function.
    ///
    /// # Parameters
    ///
    /// * `method_name` - The API method name as defined in the method settings map.
    ///   This identifies which endpoint to call and how to format the request.
    /// * `parameters` - Optional map of parameter names to values. These parameters can be:
    ///   - Path parameters (replacing placeholders in the URL path)
    ///   - Query parameters (for GET/DELETE requests)
    ///   - Body parameters (for POST requests)
    ///
    /// # Returns
    ///
    /// * `Result<T, Error>` - On success, returns the deserialized response body as type T.
    ///   On failure, returns an error that may include API error details if available.
    pub async fn do_request<T: DeserializeOwned>(
        &self,
        method_name: &'static str,
        parameters: Option<HashMap<&'static str, String>>,
    ) -> Result<T, Error> // Return Result with generic type T and our Error
    where
        T: DeserializeOwned,
    {
        let method_setting = self.method_settings.get(method_name).ok_or_else(|| {
            Error::MethodNotFound(method_name) // Use the new Error variant
        })?;

        // println!("\n--- Request for: {} ---", method_name);
        // println!("Method Setting: {:?}", method_setting);
        // println!("Input Parameters: {:?}", parameters);

        let mut params = parameters.unwrap_or_default();

        // 1. Separate Path vs. Query/Body Parameters
        let mut path_params_map = HashMap::new();
        // Identify path params based on method_setting.path_segments
        for segment in method_setting.path_segments {
            if segment.starts_with(':') {
                let param_name = &segment[1..];
                // Try to get the value. We don't remove it yet,
                // build_url_path will handle removal and error checking.
                if let Some(value) = params.get(param_name) {
                    path_params_map.insert(param_name, value.clone());
                }
                // If the param is missing here, build_url_path will error later if required.
            }
        }
        for (key, _) in &path_params_map {
            params.remove(key);
        }
        let remaining_params = params; // These are now query or body params

        let base_url_path = TradingApiSdkV4::build_url_path(method_setting, &mut path_params_map)?; // path_params_map is mutated here
        // println!("Base URL Path: {}", base_url_path);


        // 2. Prepare URL and Parameters based on HTTP Method
        let mut url_for_request = Url::parse(&base_url_path)?;
        let url_for_signature: String;
        let params_for_signature: Option<&HashMap<&'static str, String>>;
        let params_for_body: Option<&HashMap<&'static str, String>>;
        // params_for_query is implicitly handled by url_for_request


        match method_setting.http_method {
            "GET" | "DELETE" => {
                // Append remaining_params as query parameters for both request and signature URL
                if !remaining_params.is_empty() {
                    let query_string = serde_urlencoded::to_string(&remaining_params)?;
                    url_for_request.set_query(Some(&query_string));
                }
                url_for_signature = url_for_request.to_string(); // Signature uses full URL with query
                params_for_signature = None; // GET/DELETE use empty string MD5
                params_for_body = None;
                // params_for_query = None; // Already in url_for_request
                // println!("GET/DELETE - Final URL for Request: {}", url_for_request);
                // println!("GET/DELETE - URL for Signature: {}", url_for_signature);
            }
            "POST" => {
                url_for_signature = base_url_path.clone(); // Signature uses base path
                params_for_signature = if remaining_params.is_empty() { None } else { Some(&remaining_params) };
                params_for_body = if remaining_params.is_empty() { None } else { Some(&remaining_params) };
                // params_for_query = None;
                // println!("POST - URL for Request: {}", url_for_request);
                // println!("POST - URL for Signature: {}", url_for_signature);
                // println!("POST - Params for Signature/Body: {:?}", params_for_signature);
            }
            _ => {
                // This case should theoretically be unreachable if method_settings are correct
                return Err(Error::Other(format!("Unsupported HTTP method: {}", method_setting.http_method)));
            }
        }


        // 3. Generate Nonce
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_micros()
            .to_string();
        // println!("Nonce: {}", nonce);

        // 4. Generate Signature
        let signature = self.generate_signature(
            method_setting.http_method,
            &url_for_signature, // Use the correctly determined URL for signature
            &nonce,
            params_for_signature, // Pass only relevant params for signature
        )?;
        // println!("Signature: {}", signature);

        // 5. Create Headers
        let mut headers = HeaderMap::new();
        headers.insert("X-API-KEY", HeaderValue::from_str(&self.api_credentials.api_key)?);
        headers.insert("X-API-NONCE", HeaderValue::from_str(&nonce)?);
        headers.insert("X-API-SIGNATURE", HeaderValue::from_str(&signature)?);
        // Ensure Content-Type for POST requests with a body
        if method_setting.http_method == "POST" && params_for_body.is_some() {
            headers.insert("Content-Type", HeaderValue::from_static("application/x-www-form-urlencoded"));
        }
        // println!("Request Headers: {:?}", headers);


        // 6. Build and Send HTTP Request
        let mut request_builder = match method_setting.http_method {
            "GET" => self.client.get(url_for_request),
            "POST" => self.client.post(url_for_request),
            "DELETE" => self.client.delete(url_for_request),
            _ => unreachable!(), // Already handled
        };

        request_builder = request_builder.headers(headers);

        // Add body for POST requests
        if let Some(body_params) = params_for_body {
            // Use .form() for application/x-www-form-urlencoded
            request_builder = request_builder.form(body_params);
            // println!("Request Body (form): {:?}", body_params);
        }
        // Note: Query parameters for GET/DELETE are already part of the url_for_request URL

        // println!("Sending Request: {:?}", request_builder); // Debug before sending

        let response = request_builder.send().await?;
        let status_code = response.status();
        // println!("Response Status: {}", status_code);

        // 7. Handle Response
        let response_text = response.text().await?; // Read body regardless of status for potential error details

        if !status_code.is_success() {
            eprintln!("API Error Response Body: {}", response_text);
            // Use the helper function to create the API error variant
            Err(Error::api_error(status_code, response_text))
        } else {
            // println!("API Success Response Body: {}", response_text);
            // Deserialize the successful response body into type T
            let deserialized_response = from_str(&response_text)?; // Deserialize here
            Ok(deserialized_response) // Return the deserialized struct
        }
    }

    /// Generates a cryptographic signature for authenticating API requests to Bitcoin.de.
    ///
    /// This function implements the Bitcoin.de Trading API v4 signature generation process:
    /// 1. Calculates an MD5 hash of the request body parameters (for POST requests)
    /// 2. Constructs a signature string from the HTTP method, URL, API key, nonce, and MD5 hash
    /// 3. Computes an HMAC-SHA256 signature using the API secret as the key
    /// 4. Returns the hex-encoded signature for use in the X-API-SIGNATURE header
    ///
    /// # Parameters
    ///
    /// * `http_method` - The HTTP method of the request (GET, POST, DELETE)
    /// * `url_for_signature` - The complete URL path used for signature generation
    /// * `nonce` - A unique timestamp-based value to prevent replay attacks
    /// * `body_parameters` - Optional request body parameters for POST requests; None for GET/DELETE
    ///
    /// # Returns
    ///
    /// * `Result<String, Error>` - On success, returns the hex-encoded HMAC-SHA256 signature.
    ///   On failure (e.g., HMAC initialization error), returns an error.
    fn generate_signature(
        &self,
        http_method: &str,
        url_for_signature: &str, // Renamed for clarity
        nonce: &str,
        body_parameters: Option<&HashMap<&'static str, String>>, // Renamed for clarity (only for POST)
    ) -> Result<String, Error> {
        let api_key = &self.api_credentials.api_key;
        let api_secret = &self.api_credentials.api_secret;

        // 1. Prepare POST parameter MD5 hash (only if body_parameters exist)
        let post_parameter_md5_hashed_string = match body_parameters {
            Some(params) if !params.is_empty() => {
                // Sort parameters alphabetically by key
                let mut sorted_params: Vec<(&&str, &String)> = params.iter().collect();
                sorted_params.sort_by_key(|(key, _)| *key);
                // println!("Sorted Body Parameters for MD5: {:?}", sorted_params);

                // URL-encode the sorted parameters
                let url_encoded_query_string = serde_urlencoded::to_string(&sorted_params)?;
                // println!("URL Encoded Body String for MD5: {}", url_encoded_query_string);

                // Calculate MD5 hash
                let digest = md5::compute(url_encoded_query_string.as_bytes());
                let md5_hex = hex::encode(digest.0);
                // println!("Body MD5 Hash (Hex): {}", md5_hex);
                md5_hex
            }
            _ => {
                // For GET, DELETE, or POST with empty body
                let empty_md5 = "d41d8cd98f00b204e9800998ecf8427e";
                // println!("Body MD5 Hash (Empty/GET/DELETE): {}", empty_md5);
                empty_md5.to_string()
            }
        };

        // 2. Concatenate HMAC data
        let hmac_data = format!(
            "{}#{}#{}#{}#{}",
            http_method.to_uppercase(),
            url_for_signature, // Use the correct URL passed in
            api_key,
            nonce,
            post_parameter_md5_hashed_string
        );
        // println!("HMAC Data String: {}", hmac_data);

        // 3. Calculate HMAC-SHA256 signature
        type HmacSha256 = Hmac<Sha256>;
        let mut mac = HmacSha256::new_from_slice(api_secret.as_bytes())?;
        mac.update(hmac_data.as_bytes());
        let result = mac.finalize();
        let signature_bytes = result.into_bytes();

        // 4. Hex encode the signature (lowercase)
        let signature = hex::encode(signature_bytes); // hex::encode produces lowercase
        // println!("Final Signature (Hex): {}", signature);

        Ok(signature)
    }

    // --- Public API Methods (Wrappers around do_request) ---
    // Add these methods inside the impl block, after generate_signature

    /// Retrieves account information for the authenticated user.
    /// Corresponds to the `showAccountInfo` API method.
    pub async fn show_account_info(&self) -> Result<ShowAccountInfoResponse, Error> {
        self.do_request(METHOD_SHOW_ACCOUNT_INFO, None).await
    }

    /// Retrieves current trading rates for a specific trading pair.
    /// Corresponds to the `showRates` API method.
    ///
    /// # Arguments
    ///
    /// * `trading_pair` - The trading pair (e.g., "btceur") for which to retrieve rates.
    ///                    Use constants from `enums::TradingPair` and convert to lowercase string.
    pub async fn show_rates(&self, trading_pair: TradingPair) -> Result<ShowRatesResponse, Error> {
        let mut params = HashMap::new();
        params.insert(SHOW_RATES_PARAMETER_TRADING_PAIR, trading_pair.to_string().to_ascii_lowercase());
        self.do_request(METHOD_SHOW_RATES, Some(params)).await
    }

    /// Retrieves the public order book for a specific trading pair.
    /// Corresponds to the `showOrderbook` API method.
    ///
    /// # Arguments
    ///
    /// * `trading_pair` - The trading pair (e.g., "btceur").
    /// * `params` - Optional additional query parameters (e.g., "type", "amount_currency_to_trade").
    pub async fn show_orderbook(
        &self,
        trading_pair: String,
        params: Option<HashMap<&'static str, String>>,
    ) -> Result<ShowOrderbookResponse, Error> {
        let mut all_params = params.unwrap_or_default();
        // Add the trading_pair as a path parameter for do_request
        all_params.insert(SHOW_ORDERBOOK_PARAMETER_TRADING_PAIR, trading_pair);
        self.do_request(METHOD_SHOW_ORDERBOOK, Some(all_params)).await
    }

    /// Retrieves details for a specific public order.
    /// Corresponds to the `showOrderDetails` API method.
    ///
    /// # Arguments
    ///
    /// * `trading_pair` - The trading pair (e.g., "btceur").
    /// * `order_id` - The ID of the order.
    pub async fn show_order_details(
        &self,
        trading_pair: String,
        order_id: String,
    ) -> Result<ShowOrderDetailsResponse, Error> {
        let mut params = HashMap::new();
        params.insert(SHOW_ORDER_DETAILS_PARAMETER_TRADING_PAIR, trading_pair);
        params.insert(SHOW_ORDER_DETAILS_PARAMETER_ORDER_ID, order_id);
        self.do_request(METHOD_SHOW_ORDER_DETAILS, Some(params)).await
    }

    /// Creates a new order.
    /// Corresponds to the `createOrder` API method.
    ///
    /// # Arguments
    ///
    /// * `trading_pair` - The trading pair (e.g., "btceur").
    /// * `params` - Parameters for the order (e.g., "type", "max_amount_currency_to_trade", "price").
    pub async fn create_order(
        &self,
        trading_pair: String,
        mut params: HashMap<&'static str, String>, // Expect required params here
    ) -> Result<CreateOrderResponse, Error> {
        // Add the trading_pair as a path parameter for do_request
        params.insert(CREATE_ORDER_PARAMETER_TRADING_PAIR, trading_pair);
        self.do_request(METHOD_CREATE_ORDER, Some(params)).await
    }

    /// Deletes an existing order.
    /// Corresponds to the `deleteOrder` API method.
    ///
    /// # Arguments
    ///
    /// * `trading_pair` - The trading pair (e.g., "btceur").
    /// * `order_id` - The ID of the order to delete.
    pub async fn delete_order(
        &self,
        trading_pair: String,
        order_id: String,
    ) -> Result<DeleteOrderResponse, Error> {
        let mut params = HashMap::new();
        params.insert(DELETE_ORDER_PARAMETER_TRADING_PAIR, trading_pair);
        params.insert(DELETE_ORDER_PARAMETER_ORDER_ID, order_id);
        self.do_request(METHOD_DELETE_ORDER, Some(params)).await
    }

    /// Retrieves the user's orders.
    /// Corresponds to the `showMyOrders` API method.
    /// Can filter by trading pair and other criteria.
    ///
    /// # Arguments
    ///
    /// * `trading_pair` - Optional trading pair to filter by.
    ///                    Pass `None` to get orders for all pairs.
    ///                    Use constants from `enums::TradingPair` and convert to lowercase string.
    /// * `params` - Optional additional query parameters (e.g., "type", "state", "page").
    pub async fn show_my_orders(
        &self,
        trading_pair: Option<String>,
        params: Option<HashMap<&'static str, String>>,
    ) -> Result<ShowMyOrdersResponse, Error> {
        let mut all_params = params.unwrap_or_default();
        // Add trading_pair as a path parameter if provided
        if let Some(pair) = trading_pair {
            // This parameter name needs to match the one used as the placeholder in method_settings.rs
            all_params.insert(SHOW_MY_ORDERS_PARAMETER_TRADING_PAIR, pair);
        }
        // Note: The do_request function and method_settings need to correctly handle
        // the optional path parameter for this endpoint.
        self.do_request(METHOD_SHOW_MY_ORDERS, Some(all_params)).await
    }

    /// Retrieves the user's trades.
    /// Corresponds to the `showMyTrades` API method.
    /// Can filter by trading pair and other criteria.
    ///
    /// # Arguments
    ///
    /// * `trading_pair` - Optional trading pair to filter by.
    ///                    Pass `None` to get trades for all pairs.
    ///                    Use constants from `enums::TradingPair` and convert to lowercase string.
    /// * `params` - Optional additional query parameters (e.g., "type", "state", "page").
    pub async fn show_my_trades(
        &self,
        trading_pair: Option<String>,
        params: Option<HashMap<&'static str, String>>,
    ) -> Result<ShowMyTradesResponse, Error> {
        let mut all_params = params.unwrap_or_default();
        // Add trading_pair as a path parameter if provided
        if let Some(pair) = trading_pair {
            all_params.insert(SHOW_MY_TRADES_PARAMETER_TRADING_PAIR, pair);
        }
        // Note: Similar handling needed in do_request for optional path param
        self.do_request(METHOD_SHOW_MY_TRADES, Some(all_params)).await
    }

    /// Retrieves details for one of your trades.
    /// Corresponds to the `showMyTradeDetails` API method.
    ///
    /// # Arguments
    ///
    /// * `trading_pair` - The trading pair (e.g., "btceur").
    /// * `trade_id` - The ID of the trade.
    pub async fn show_my_trade_details(
        &self,
        trading_pair: String,
        trade_id: String,
    ) -> Result<ShowMyTradeDetailsResponse, Error> {
        let mut params = HashMap::new();
        // These parameter names need to match the ones used as placeholders in method_settings.rs
        params.insert(SHOW_MY_TRADE_DETAILS_PARAMETER_TRADING_PAIR, trading_pair);
        params.insert(SHOW_MY_TRADE_DETAILS_PARAMETER_TRADE_ID, trade_id);
        self.do_request(METHOD_SHOW_MY_TRADE_DETAILS, Some(params)).await
    }

    /// Executes a trade against a specific order.
    /// Corresponds to the `executeTrade` API method.
    ///
    /// # Arguments
    ///
    /// * `trading_pair` - The trading pair (e.g., "btceur").
    /// * `order_id` - The ID of the order to trade against.
    /// * `params` - Parameters for the trade execution (e.g., "type", "amount_currency_to_trade").
    pub async fn execute_trade(
        &self,
        trading_pair: String,
        order_id: String,
        mut params: HashMap<&'static str, String>, // Expect required params here
    ) -> Result<ExecuteTradeResponse, Error> {
        // Add the trading_pair and order_id as path parameters for do_request
        params.insert(EXECUTE_TRADE_PARAMETER_TRADING_PAIR, trading_pair);
        params.insert(EXECUTE_TRADE_PARAMETER_ORDER_ID, order_id);
        self.do_request(METHOD_EXECUTE_TRADE, Some(params)).await
    }

    /// Marks a trade as paid.
    /// Corresponds to the `markTradeAsPaid` API method.
    ///
    /// # Arguments
    ///
    /// * `trading_pair` - The trading pair (e.g., "btceur").
    /// * `trade_id` - The ID of the trade.
    /// * `volume_currency_to_pay_after_fee` - The EUR volume after fee.
    pub async fn mark_trade_as_paid(
        &self,
        trading_pair: String,
        trade_id: String,
        volume_currency_to_pay_after_fee: String, // Use String or Decimal and convert
    ) -> Result<MarkTradeAsPaidResponse, Error> {
        let mut params = HashMap::new();
        // Add path parameters
        params.insert(MARK_TRADE_AS_PAID_PARAMETER_TRADING_PAIR, trading_pair);
        params.insert(MARK_TRADE_AS_PAID_PARAMETER_TRADE_ID, trade_id);
        // Add body parameter
        params.insert(MARK_TRADE_AS_PAID_PARAMETER_VOLUME_CURRENCY_TO_PAY_AFTER_FEE, volume_currency_to_pay_after_fee);
        self.do_request(METHOD_MARK_TRADE_AS_PAID, Some(params)).await
    }

    /// Marks a trade as payment received.
    /// Corresponds to the `markTradeAsPaymentReceived` API method.
    ///
    /// # Arguments
    ///
    /// * `trading_pair` - The trading pair (e.g., "btceur").
    /// * `trade_id` - The ID of the trade.
    /// * `params` - Parameters for marking as received (e.g., "volume_currency_to_pay_after_fee", "rating").
    pub async fn mark_trade_as_payment_received(
        &self,
        trading_pair: String,
        trade_id: String,
        mut params: HashMap<&'static str, String>, // Expect required params here
    ) -> Result<MarkTradeAsPaymentReceivedResponse, Error> {
        // Add path parameters
        params.insert(MARK_TRADE_AS_PAYMENT_RECEIVED_PARAMETER_TRADING_PAIR, trading_pair);
        params.insert(MARK_TRADE_AS_PAYMENT_RECEIVED_PARAMETER_TRADE_ID, trade_id);
        // Call do_request with combined params
        self.do_request(METHOD_MARK_TRADE_AS_PAYMENT_RECEIVED, Some(params)).await
    }

    /// Adds a rating to a trade partner.
    /// Corresponds to the `addTradeRating` API method.
    ///
    /// # Arguments
    ///
    /// * `trading_pair` - The trading pair (e.g., "btceur").
    /// * `trade_id` - The ID of the trade.
    /// * `rating` - The rating ("positive", "neutral", "negative").
    pub async fn add_trade_rating(
        &self,
        trading_pair: String,
        trade_id: String,
        rating: String, // Use String or an appropriate Enum
    ) -> Result<AddTradeRatingResponse, Error> {
        let mut params = HashMap::new();
        // Add path parameters
        params.insert(ADD_TRADE_RATING_PARAMETER_TRADING_PAIR, trading_pair);
        params.insert(ADD_TRADE_RATING_PARAMETER_TRADE_ID, trade_id);
        // Add body parameter
        params.insert(ADD_TRADE_RATING_PARAMETER_RATING, rating);
        self.do_request(METHOD_ADD_TRADE_RATING, Some(params)).await
    }


    /// Retrieves your account ledger entries for a specific currency.
    /// Corresponds to the `showAccountLedger` API method.
    /// Can filter by type, date range, or page.
    ///
    /// # Arguments
    ///
    /// * `currency` - The currency to list ledger entries for (e.g., "BTC").
    /// * `params` - Optional additional query parameters (e.g., "type", "datetime_start", "page").
    pub async fn show_account_ledger(
        &self,
        currency: String,
        params: Option<HashMap<&'static str, String>>,
    ) -> Result<ShowAccountLedgerResponse, Error> {
        let mut all_params = params.unwrap_or_default();
        // Add the currency as a path parameter for do_request
        all_params.insert(SHOW_ACCOUNT_LEDGER_PARAMETER_CURRENCY, currency);
        self.do_request(METHOD_SHOW_ACCOUNT_LEDGER, Some(all_params)).await
    }

    /// Retrieves the permissions associated with the API key.
    /// Corresponds to the `showPermissions` API method.
    pub async fn show_permissions(&self) -> Result<ShowPermissionsResponse, Error> {
        self.do_request(METHOD_SHOW_PERMISSIONS, None).await
    }

    /// Creates a new cryptocurrency withdrawal.
    /// Corresponds to the `createWithdrawal` API method.
    ///
    /// # Arguments
    ///
    /// * `currency` - The currency to withdraw (e.g., "BTC").
    /// * `params` - Parameters for the withdrawal (e.g., "amount", "address", "network_fee").
    pub async fn create_withdrawal(
        &self,
        currency: String,
        mut params: HashMap<&'static str, String>, // Expect required params here
    ) -> Result<CreateWithdrawalResponse, Error> {
        // Add the currency as a path parameter for do_request
        params.insert(CREATE_WITHDRAWAL_PARAMETER_CURRENCY, currency);
        self.do_request(METHOD_CREATE_WITHDRAWAL, Some(params)).await
    }

    /// Deletes an existing withdrawal request.
    /// Corresponds to the `deleteWithdrawal` API method.
    ///
    /// # Arguments
    ///
    /// * `currency` - The currency of the withdrawal (e.g., "BTC").
    /// * `withdrawal_id` - The ID of the withdrawal to delete.
    pub async fn delete_withdrawal(
        &self,
        currency: String,
        withdrawal_id: String,
    ) -> Result<DeleteWithdrawalResponse, Error> {
        let mut params = HashMap::new();
        // Add path parameters
        params.insert(DELETE_WITHDRAWAL_PARAMETER_CURRENCY, currency);
        params.insert(DELETE_WITHDRAWAL_PARAMETER_WITHDRAWAL_ID, withdrawal_id);
        self.do_request(METHOD_DELETE_WITHDRAWAL, Some(params)).await
    }

    /// Retrieves details for a specific withdrawal.
    /// Corresponds to the `showWithdrawal` API method.
    ///
    /// # Arguments
    ///
    /// * `currency` - The currency of the withdrawal (e.g., "BTC").
    /// * `withdrawal_id` - The ID of the withdrawal.
    pub async fn show_withdrawal(
        &self,
        currency: String,
        withdrawal_id: String,
    ) -> Result<ShowWithdrawalResponse, Error> {
        let mut params = HashMap::new();
        // Add path parameters
        params.insert(SHOW_WITHDRAWAL_PARAMETER_CURRENCY, currency);
        params.insert(SHOW_WITHDRAWAL_PARAMETER_WITHDRAWAL_ID, withdrawal_id);
        self.do_request(METHOD_SHOW_WITHDRAWAL, Some(params)).await
    }

    /// Retrieves your cryptocurrency withdrawals for a specific currency.
    /// Corresponds to the `showWithdrawals` API method.
    /// Can filter by address or page.
    ///
    /// # Arguments
    ///
    /// * `currency` - The currency to list withdrawals for (e.g., "BTC").
    /// * `params` - Optional additional query parameters (e.g., "address", "page").
    pub async fn show_withdrawals(
        &self,
        currency: String,
        params: Option<HashMap<&'static str, String>>,
    ) -> Result<ShowWithdrawalsResponse, Error> {
        let mut all_params = params.unwrap_or_default();
        // Add the currency as a path parameter for do_request
        all_params.insert(SHOW_WITHDRAWALS_PARAMETER_CURRENCY, currency);
        self.do_request(METHOD_SHOW_WITHDRAWALS, Some(all_params)).await
    }

    /// Retrieves the minimum network fee for a withdrawal in a specific currency.
    /// Corresponds to the `showWithdrawalMinNetworkFee` API method.
    ///
    /// # Arguments
    ///
    /// * `currency` - The currency (e.g., "BTC").
    pub async fn show_withdrawal_min_network_fee(
        &self,
        currency: String,
    ) -> Result<ShowWithdrawalMinNetworkFeeResponse, Error> {
        let mut params = HashMap::new();
        // Add the currency as a path parameter for do_request
        params.insert(SHOW_WITHDRAWAL_PARAMETER_MIN_NETWORK_FEE_CURRENCY, currency);
        self.do_request(METHOD_SHOW_WITHDRAWAL_MIN_NETWORK_FEE, Some(params)).await
    }

    /// Requests a new deposit address for a specific currency.
    /// Corresponds to the `requestDepositAddress` API method.
    ///
    /// # Arguments
    ///
    /// * `currency` - The currency (e.g., "BTC").
    /// * `comment` - Optional comment for the new address.
    pub async fn request_deposit_address(
        &self,
        currency: String,
        comment: Option<String>,
    ) -> Result<RequestDepositAddressResponse, Error> {
        let mut params = HashMap::new();
        // Add the currency as a path parameter for do_request
        params.insert(REQUEST_DEPOSIT_ADDRESS_PARAMETER_CURRENCY, currency);
        // Add comment as a body parameter if provided
        if let Some(c) = comment {
            params.insert("comment", c); // Use literal "comment" parameter name
        }
        self.do_request(METHOD_REQUEST_DEPOSIT_ADDRESS, Some(params)).await
    }


    /// Retrieves details for a specific deposit.
    /// Corresponds to the `showDeposit` API method.
    ///
    /// # Arguments
    ///
    /// * `currency` - The currency of the deposit (e.g., "BTC").
    /// * `deposit_id` - The ID of the deposit.
    pub async fn show_deposit(
        &self,
        currency: String,
        deposit_id: String,
    ) -> Result<ShowDepositResponse, Error> {
        let mut params = HashMap::new();
        // Add path parameters
        params.insert(SHOW_DEPOSIT_PARAMETER_CURRENCY, currency);
        params.insert(SHOW_DEPOSIT_PARAMETER_DEPOSIT_ID, deposit_id);
        self.do_request(METHOD_SHOW_DEPOSIT, Some(params)).await
    }

    /// Retrieves your cryptocurrency deposits for a specific currency.
    /// Corresponds to the `showDeposits` API method.
    /// Can filter by address or page.
    ///
    /// # Arguments
    ///
    /// * `currency` - The currency to list deposits for (e.g., "BTC").
    /// * `params` - Optional additional query parameters (e.g., "address", "page").
    pub async fn show_deposits(
        &self,
        currency: String,
        params: Option<HashMap<&'static str, String>>,
    ) -> Result<ShowDepositsResponse, Error> {
        let mut all_params = params.unwrap_or_default();
        // Add the currency as a path parameter for do_request
        all_params.insert(SHOW_DEPOSITS_PARAMETER_CURRENCY, currency);
        self.do_request(METHOD_SHOW_DEPOSITS, Some(all_params)).await
    }

    /// Creates a new outgoing address in the address pool.
    /// Corresponds to the `createOutgoingAddress` API method.
    ///
    /// # Arguments
    ///
    /// * `currency` - The currency (e.g., "BTC").
    /// * `params` - Parameters for the outgoing address (e.g., "recipient_address", "recipient_purpose", "comment").
    pub async fn create_outgoing_address(
        &self,
        currency: String,
        mut params: HashMap<&'static str, String>, // Expect required params here
    ) -> Result<BasicSuccessResponse, Error> { // Assuming BasicSuccessResponse
        // Add the currency as a path parameter for do_request
        params.insert(CREATE_OUTGOING_ADDRESS_PARAMETER_CURRENCY, currency);
        self.do_request(METHOD_CREATE_OUTGOING_ADDRESS, Some(params)).await
    }

    /// Deletes an outgoing address from the address pool.
    /// Corresponds to the `deleteOutgoingAddress` API method.
    ///
    /// # Arguments
    ///
    /// * `currency` - The currency (e.g., "BTC").
    /// * `address_id` - The ID of the address to delete.
    pub async fn delete_outgoing_address(
        &self,
        currency: String,
        address_id: String,
    ) -> Result<BasicSuccessResponse, Error> { // Assuming BasicSuccessResponse
        let mut params = HashMap::new();
        // Add path parameters
        params.insert(DELETE_OUTGOING_ADDRESS_PARAMETER_CURRENCY, currency);
        params.insert(DELETE_OUTGOING_ADDRESS_PARAMETER_ADDRESS_ID, address_id);
        self.do_request(METHOD_DELETE_OUTGOING_ADDRESS, Some(params)).await
    }

    /// Retrieves your outgoing addresses for a specific currency.
    /// Corresponds to the `showOutgoingAddresses` API method.
    /// Can filter by page.
    ///
    /// # Arguments
    ///
    /// * `currency` - The currency (e.g., "BTC").
    /// * `params` - Optional additional query parameters (e.g., "page").
    pub async fn show_outgoing_addresses(
        &self,
        currency: String,
        params: Option<HashMap<&'static str, String>>,
    ) -> Result<ShowOutgoingAddressesResponse, Error> {
        let mut all_params = params.unwrap_or_default();
        // Add the currency as a path parameter for do_request
        all_params.insert(SHOW_OUTGOING_ADDRESS_PARAMETER_CURRENCY, currency);
        self.do_request(METHOD_SHOW_OUTGOING_ADDRESSES, Some(all_params)).await
    }

    /// Retrieves the public trade history for a specific trading pair.
    /// Corresponds to the `showPublicTradeHistory` API method.
    /// Can filter by `since_tid`.
    ///
    /// # Arguments
    ///
    /// * `trading_pair` - The trading pair (e.g., "btceur").
    /// * `since_tid` - Optional parameter to show trades with an ID greater than this value.
    pub async fn show_public_trade_history(
        &self,
        trading_pair: String,
        since_tid: Option<String>, // Use String as query param value
    ) -> Result<ShowPublicTradeHistoryResponse, Error> {
        let mut params = HashMap::new();
        // Add the trading_pair as a path parameter for do_request
        params.insert(SHOW_PUBLIC_TRADE_HISTORY_PARAMETER_TRADING_PAIR, trading_pair);
        // Add since_tid as a query parameter if provided
        if let Some(tid) = since_tid {
            params.insert("since_tid", tid); // Use literal "since_tid" parameter name
        }
        self.do_request(METHOD_SHOW_PUBLIC_TRADE_HISTORY, Some(params)).await
    }

    /// Retrieves the compact order book for a specific trading pair.
    /// Corresponds to the `showOrderbookCompact` API method.
    ///
    /// # Arguments
    ///
    /// * `trading_pair` - The trading pair (e.g., "btceur").
    pub async fn show_orderbook_compact(
        &self,
        trading_pair: String,
    ) -> Result<ShowOrderbookCompactResponse, Error> {
        let mut params = HashMap::new();
        // Add the trading_pair as a path parameter for do_request
        params.insert(SHOW_ORDER_BOOK_COMPACT_PARAMETER_TRADING_PAIR, trading_pair);
        self.do_request(METHOD_SHOW_ORDERBOOK_COMPACT, Some(params)).await
    }

    /// Adds an address to the address pool for a specific currency.
    /// Corresponds to the `addToAddressPool` API method.
    ///
    /// # Arguments
    ///
    /// * `currency` - The currency (e.g., "BTC").
    /// * `address` - The address to add.
    pub async fn add_to_address_pool(
        &self,
        currency: String,
        address: String,
    ) -> Result<BasicSuccessResponse, Error> { // Assuming BasicSuccessResponse
        let mut params = HashMap::new();
        // Add the currency as a path parameter for do_request
        params.insert(ADD_TO_ADDRESS_POOL_PARAMETER_CURRENCY, currency);
        // Add address as a body parameter
        params.insert(ADD_TO_ADDRESS_POOL_PARAMETER_ADDRESS, address);
        self.do_request(METHOD_ADD_TO_ADDRESS_POOL, Some(params)).await
    }

    /// Removes an address from the address pool for a specific currency.
    /// Corresponds to the `removeFromAddressPool` API method.
    ///
    /// # Arguments
    ///
    /// * `currency` - The currency (e.g., "BTC").
    /// * `address` - The address to remove (used as ID in path).
    pub async fn remove_from_address_pool(
        &self,
        currency: String,
        address: String,
    ) -> Result<BasicSuccessResponse, Error> { // Assuming BasicSuccessResponse
        let mut params = HashMap::new();
        // Add path parameters (address is a path parameter here)
        params.insert(REMOVE_FROM_ADDRESS_POOL_PARAMETER_CURRENCY, currency);
        params.insert(REMOVE_FROM_ADDRESS_POOL_PARAMETER_ADDRESS, address);
        self.do_request(METHOD_REMOVE_FROM_ADDRESS_POOL, Some(params)).await
    }

    /// Lists addresses in the address pool for a specific currency.
    /// Corresponds to the `listAddressPool` API method.
    /// Shares the response type with `showOutgoingAddresses`.
    ///
    /// # Arguments
    ///
    /// * `currency` - The currency (e.g., "BTC").
    /// * `page` - Optional page number.
    pub async fn list_address_pool(
        &self,
        currency: String,
        page: Option<String>, // Use String as query param value
    ) -> Result<ShowOutgoingAddressesResponse, Error> { // Shares response struct
        let mut params = HashMap::new();
        // Add the currency as a path parameter for do_request
        params.insert(LIST_ADDRESS_POOL_PARAMETER_CURRENCY, currency);
        // Add page as a query parameter if provided
        if let Some(p) = page {
            params.insert("page", p); // Use literal "page" parameter name
        }
        self.do_request(METHOD_LIST_ADDRESS_POOL, Some(params)).await
    }

    /// Marks coins as transferred for a crypto-to-crypto trade.
    /// Corresponds to the `markCoinsAsTransferred` API method.
    ///
    /// # Arguments
    ///
    /// * `trading_pair` - The trading pair (e.g., "btceur").
    /// * `trade_id` - The ID of the trade.
    /// * `amount_currency_to_trade_after_fee` - The amount after fee.
    pub async fn mark_coins_as_transferred(
        &self,
        trading_pair: String,
        trade_id: String,
        amount_currency_to_trade_after_fee: String, // Use String or Decimal and convert
    ) -> Result<BasicSuccessResponse, Error> { // Assuming BasicSuccessResponse
        let mut params = HashMap::new();
        // Add path parameters
        params.insert(MARK_COINS_AS_TRANSFERRED_PARAMETER_TRADING_PAIR, trading_pair);
        params.insert(MARK_COINS_AS_TRANSFERRED_PARAMETER_TRADE_ID, trade_id);
        // Add body parameter
        params.insert(MARK_COINS_AS_TRANSFERRED_PARAMETER_AMOUNT_CURRENCY_TO_TRADE_AFTER_FEE, amount_currency_to_trade_after_fee);
        self.do_request(METHOD_MARK_COINS_AS_TRANSFERRED, Some(params)).await
    }

    /// Marks coins as received for a crypto-to-crypto trade.
    /// Corresponds to the `markCoinsAsReceived` API method.
    ///
    /// # Arguments
    ///
    /// * `trading_pair` - The trading pair (e.g., "btceur").
    /// * `trade_id` - The ID of the trade.
    /// * `params` - Parameters for marking as received (e.g., "amount_currency_to_trade_after_fee", "rating").
    pub async fn mark_coins_as_received(
        &self,
        trading_pair: String,
        trade_id: String,
        mut params: HashMap<&'static str, String>, // Expect required params here
    ) -> Result<BasicSuccessResponse, Error> { // Assuming BasicSuccessResponse
        // Add path parameters
        params.insert(MARK_COINS_AS_RECEIVED_PARAMETER_TRADING_PAIR, trading_pair);
        params.insert(MARK_COINS_AS_RECEIVED_PARAMETER_TRADE_ID, trade_id);
        // Call do_request with combined params
        self.do_request(METHOD_MARK_COINS_AS_RECEIVED, Some(params)).await
    }

}