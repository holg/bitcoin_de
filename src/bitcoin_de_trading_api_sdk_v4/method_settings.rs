// bitcoin_de_trading_api_sdk_v4/method_settings.rs
use std::collections::HashMap;
use std::sync::LazyLock; // Use modern LazyLock for static initialization

/// Constants for method names and parameter names to ensure consistency across the Bitcoin.de API v4 SDK.
///
/// This module provides a centralized location for all constant values used throughout the SDK,
/// including API endpoints, HTTP methods, method names, and parameter identifiers. Using these
/// constants instead of string literals helps prevent typos and ensures consistency when
/// referencing API components.
///
/// The constants are organized into logical groups:
/// - Base URI for the API
/// - HTTP methods (GET, POST, DELETE)
/// - Method names that match the keys in METHOD_SETTINGS
/// - Parameter names for each API endpoint, grouped by functionality
///
/// By using these constants in method calls and parameter validation, the SDK maintains
/// a consistent interface to the Bitcoin.de API v4.
pub mod constants {
    // Base URI for the API v4
    pub const API_BASE_URI: &str = "https://api.bitcoin.de/v4"; // Ensure no trailing slash

    // HTTP Methods
    pub const HTTP_METHOD_GET: &str = "GET";
    pub const HTTP_METHOD_POST: &str = "POST";
    pub const HTTP_METHOD_DELETE: &str = "DELETE";

    // Method Names (ensure these match the keys used in METHOD_SETTINGS)
    pub const METHOD_SHOW_ORDERBOOK: &str = "showOrderbook";
    pub const METHOD_SHOW_ORDER_DETAILS: &str = "showOrderDetails";
    pub const METHOD_CREATE_ORDER: &str = "createOrder";
    pub const METHOD_DELETE_ORDER: &str = "deleteOrder";
    pub const METHOD_SHOW_MY_ORDERS: &str = "showMyOrders";
    pub const METHOD_SHOW_MY_ORDER_DETAILS: &str = "showMyOrderDetails";
    pub const METHOD_EXECUTE_TRADE: &str = "executeTrade";
    pub const METHOD_SHOW_MY_TRADES: &str = "showMyTrades";
    pub const METHOD_SHOW_MY_TRADE_DETAILS: &str = "showMyTradeDetails";
    pub const METHOD_MARK_TRADE_AS_PAID: &str = "markTradeAsPaid";
    pub const METHOD_MARK_TRADE_AS_PAYMENT_RECEIVED: &str = "markTradeAsPaymentReceived";
    pub const METHOD_ADD_TRADE_RATING: &str = "addTradeRating";
    pub const METHOD_SHOW_ACCOUNT_INFO: &str = "showAccountInfo";
    pub const METHOD_SHOW_ACCOUNT_LEDGER: &str = "showAccountLedger";
    pub const METHOD_SHOW_PERMISSIONS: &str = "showPermissions";
    pub const METHOD_CREATE_WITHDRAWAL: &str = "createWithdrawal";
    pub const METHOD_DELETE_WITHDRAWAL: &str = "deleteWithdrawal";
    pub const METHOD_SHOW_WITHDRAWAL: &str = "showWithdrawal";
    pub const METHOD_SHOW_WITHDRAWALS: &str = "showWithdrawals";
    pub const METHOD_SHOW_WITHDRAWAL_MIN_NETWORK_FEE: &str = "showWithdrawalMinNetworkFee";
    pub const METHOD_SHOW_OUTGOING_ADDRESSES: &str = "showOutgoingAddresses"; // Corrected name based on usage
    pub const METHOD_REQUEST_DEPOSIT_ADDRESS: &str = "requestDepositAddress";
    pub const METHOD_SHOW_DEPOSIT: &str = "showDeposit";
    pub const METHOD_SHOW_DEPOSITS: &str = "showDeposits";
    pub const METHOD_SHOW_ORDERBOOK_COMPACT: &str = "showOrderbookCompact";
    pub const METHOD_SHOW_PUBLIC_TRADE_HISTORY: &str = "showPublicTradeHistory";
    pub const METHOD_SHOW_RATES: &str = "showRates";
    pub const METHOD_ADD_TO_ADDRESS_POOL: &str = "addToAddressPool";
    pub const METHOD_REMOVE_FROM_ADDRESS_POOL: &str = "removeFromAddressPool";
    pub const METHOD_LIST_ADDRESS_POOL: &str = "listAddressPool";
    pub const METHOD_MARK_COINS_AS_TRANSFERRED: &str = "markCoinsAsTransferred";
    pub const METHOD_MARK_COINS_AS_RECEIVED: &str = "markCoinsAsReceived";
    // Note: CREATE_OUTGOING_ADDRESS and DELETE_OUTGOING_ADDRESS seem missing from API docs provided, but kept if used elsewhere.
    pub const METHOD_CREATE_OUTGOING_ADDRESS: &str = "createOutgoingAddress";
    pub const METHOD_DELETE_OUTGOING_ADDRESS: &str = "deleteOutgoingAddress";


    // Parameter Names (used in MethodSetting definitions and potentially for validation)
    // Orderbook
    pub const SHOW_ORDERBOOK_PARAMETER_TRADING_PAIR: &str = "trading_pair";
    pub const SHOW_ORDERBOOK_PARAMETER_TYPE: &str = "type";
    // Order Details
    pub const SHOW_ORDER_DETAILS_PARAMETER_TRADING_PAIR: &str = "trading_pair";
    pub const SHOW_ORDER_DETAILS_PARAMETER_ORDER_ID: &str = "order_id";
    // Create Order
    pub const CREATE_ORDER_PARAMETER_TRADING_PAIR: &str = "trading_pair";
    pub const CREATE_ORDER_PARAMETER_TYPE: &str = "type";
    pub const CREATE_ORDER_PARAMETER_MAX_AMOUNT: &str = "max_amount_currency_to_trade"; // Definition was here
    pub const CREATE_ORDER_PARAMETER_PRICE: &str = "price";
    pub const CREATE_ORDER_PARAMETER_ORDER_PAYMENT_OPTIONS: &str = "payment_option"; // Optional
    // Delete Order
    pub const DELETE_ORDER_PARAMETER_TRADING_PAIR: &str = "trading_pair";
    pub const DELETE_ORDER_PARAMETER_ORDER_ID: &str = "order_id";
    // Show My Orders
    pub const SHOW_MY_ORDERS_PARAMETER_TRADING_PAIR: &str = "trading_pair"; // Optional path param - Definition was here
    // Show My Order Details
    pub const SHOW_MY_ORDER_DETAILS_PARAMETER_TRADING_PAIR: &str = "trading_pair";
    pub const SHOW_MY_ORDER_DETAILS_PARAMETER_ORDER_ID: &str = "order_id"; // Definition was here
    // Execute Trade
    pub const EXECUTE_TRADE_PARAMETER_TRADING_PAIR: &str = "trading_pair";
    pub const EXECUTE_TRADE_PARAMETER_ORDER_ID: &str = "order_id";
    pub const EXECUTE_TRADE_PARAMETER_TYPE: &str = "type";
    pub const EXECUTE_TRADE_PARAMETER_AMOUNT_CURRENCY_TO_TRADE: &str = "amount_currency_to_trade";
    // Show My Trades
    pub const SHOW_MY_TRADES_PARAMETER_TRADING_PAIR: &str = "trading_pair"; // Optional path param - Definition was here
    // Show My Trade Details
    pub const SHOW_MY_TRADE_DETAILS_PARAMETER_TRADING_PAIR: &str = "trading_pair";
    pub const SHOW_MY_TRADE_DETAILS_PARAMETER_TRADE_ID: &str = "trade_id";
    // Mark Trade As Paid
    pub const MARK_TRADE_AS_PAID_PARAMETER_TRADING_PAIR: &str = "trading_pair";
    pub const MARK_TRADE_AS_PAID_PARAMETER_TRADE_ID: &str = "trade_id";
    pub const MARK_TRADE_AS_PAID_PARAMETER_VOLUME_CURRENCY_TO_PAY_AFTER_FEE: &str = "volume_currency_to_pay_after_fee";
    // Mark Trade As Payment Received
    pub const MARK_TRADE_AS_PAYMENT_RECEIVED_PARAMETER_TRADING_PAIR: &str = "trading_pair";
    pub const MARK_TRADE_AS_PAYMENT_RECEIVED_PARAMETER_TRADE_ID: &str = "trade_id";
    pub const MARK_TRADE_AS_PAYMENT_RECEIVED_PARAMETER_VOLUME_CURRENCY_TO_PAY_AFTER_FEE: &str = "volume_currency_to_pay_after_fee";
    pub const MARK_TRADE_AS_PAYMENT_RECEIVED_PARAMETER_RATING: &str = "rating";
    pub const MARK_TRADE_AS_PAYMENT_RECEIVED_PARAMETER_IS_PAID_FROM_CORRECT_BANK_ACCOUNT: &str = "is_paid_from_correct_bank_account";
    // Add Trade Rating
    pub const ADD_TRADE_RATING_PARAMETER_TRADING_PAIR: &str = "trading_pair";
    pub const ADD_TRADE_RATING_PARAMETER_TRADE_ID: &str = "trade_id";
    pub const ADD_TRADE_RATING_PARAMETER_RATING: &str = "rating";
    // Show Account Ledger
    pub const SHOW_ACCOUNT_LEDGER_PARAMETER_CURRENCY: &str = "currency";
    // Create Withdrawal
    pub const CREATE_WITHDRAWAL_PARAMETER_CURRENCY: &str = "currency";
    pub const CREATE_WITHDRAWAL_PARAMETER_AMOUNT: &str = "amount";
    pub const CREATE_WITHDRAWAL_PARAMETER_ADDRESS: &str = "address";
    pub const CREATE_WITHDRAWAL_PARAMETER_NETWORK_FEE: &str = "network_fee";
    // Delete Withdrawal
    pub const DELETE_WITHDRAWAL_PARAMETER_CURRENCY: &str = "currency";
    pub const DELETE_WITHDRAWAL_PARAMETER_WITHDRAWAL_ID: &str = "withdrawal_id";
    // Show Withdrawal
    pub const SHOW_WITHDRAWAL_PARAMETER_CURRENCY: &str = "currency";
    pub const SHOW_WITHDRAWAL_PARAMETER_WITHDRAWAL_ID: &str = "withdrawal_id";
    // Show Withdrawals
    pub const SHOW_WITHDRAWALS_PARAMETER_CURRENCY: &str = "currency";
    // Show Withdrawal Min Network Fee
    pub const SHOW_WITHDRAWAL_PARAMETER_MIN_NETWORK_FEE_CURRENCY: &str = "currency";
    // Request Deposit Address
    pub const REQUEST_DEPOSIT_ADDRESS_PARAMETER_CURRENCY: &str = "currency";
    // Show Deposit
    pub const SHOW_DEPOSIT_PARAMETER_CURRENCY: &str = "currency";
    pub const SHOW_DEPOSIT_PARAMETER_DEPOSIT_ID: &str = "deposit_id";
    // Show Deposits
    pub const SHOW_DEPOSITS_PARAMETER_CURRENCY: &str = "currency";
    // Show Outgoing Addresses
    pub const SHOW_OUTGOING_ADDRESS_PARAMETER_CURRENCY: &str = "currency";
    // Show Public Trade History
    pub const SHOW_PUBLIC_TRADE_HISTORY_PARAMETER_TRADING_PAIR: &str = "trading_pair";
    // Show Orderbook Compact
    pub const SHOW_ORDER_BOOK_COMPACT_PARAMETER_TRADING_PAIR: &str = "trading_pair";
    // Show Rates
    pub const SHOW_RATES_PARAMETER_TRADING_PAIR: &str = "trading_pair";
    // Add To Address Pool
    pub const ADD_TO_ADDRESS_POOL_PARAMETER_CURRENCY: &str = "currency";
    pub const ADD_TO_ADDRESS_POOL_PARAMETER_ADDRESS: &str = "address";
    // Remove From Address Pool
    pub const REMOVE_FROM_ADDRESS_POOL_PARAMETER_CURRENCY: &str = "currency";
    pub const REMOVE_FROM_ADDRESS_POOL_PARAMETER_ADDRESS: &str = "address"; // Used as ID in path
    // List Address Pool
    pub const LIST_ADDRESS_POOL_PARAMETER_CURRENCY: &str = "currency";
    // Mark Coins As Transferred
    pub const MARK_COINS_AS_TRANSFERRED_PARAMETER_TRADING_PAIR: &str = "trading_pair";
    pub const MARK_COINS_AS_TRANSFERRED_PARAMETER_TRADE_ID: &str = "trade_id";
    pub const MARK_COINS_AS_TRANSFERRED_PARAMETER_AMOUNT_CURRENCY_TO_TRADE_AFTER_FEE: &str = "amount_currency_to_trade_after_fee";
    // Mark Coins As Received
    pub const MARK_COINS_AS_RECEIVED_PARAMETER_TRADING_PAIR: &str = "trading_pair";
    pub const MARK_COINS_AS_RECEIVED_PARAMETER_TRADE_ID: &str = "trade_id";
    pub const MARK_COINS_AS_RECEIVED_PARAMETER_AMOUNT_CURRENCY_TO_TRADE_AFTER_FEE: &str = "amount_currency_to_trade_after_fee";
    pub const MARK_COINS_AS_RECEIVED_PARAMETER_RATING: &str = "rating";
    // Create Outgoing Address (If needed)
    pub const CREATE_OUTGOING_ADDRESS_PARAMETER_CURRENCY: &str = "currency";
    pub const CREATE_OUTGOING_ADDRESS_PARAMETER_RECIPIENT_ADDRESS: &str = "recipient_address";
    pub const CREATE_OUTGOING_ADDRESS_PARAMETER_RECIPIENT_PURPOSE: &str = "recipient_purpose";
    pub const CREATE_OUTGOING_ADDRESS_PARAMETER_COMMENT: &str = "comment";
    // Delete Outgoing Address (If needed)
    pub const DELETE_OUTGOING_ADDRESS_PARAMETER_CURRENCY: &str = "currency";
    pub const DELETE_OUTGOING_ADDRESS_PARAMETER_ADDRESS_ID: &str = "address_id";
}

/// Defines the specific settings for a single Bitcoin.de API v4 method.
/// This includes the HTTP verb, URL path structure, and expected parameters.
#[derive(Debug, Clone)] // Added Clone for potential future use, Debug is essential
pub struct MethodSetting {
    /// The HTTP method required for the API call (e.g., "GET", "POST", "DELETE").
    pub http_method: &'static str,

    /// An ordered list of path segments that form the URL path after the base URI (`/v4/`).
    /// Segments starting with ':' are placeholders (e.g., ":trading_pair", ":order_id")
    /// that will be replaced by values from the input parameters.
    /// Literal segments are used directly (e.g., "orders", "rates", "history").
    pub path_segments: &'static [&'static str],

    /// The name of the parameter expected to contain the trading pair value (e.g., "trading_pair").
    /// Used for validation and potentially identifying the parameter if the placeholder name differs.
    /// Set to `None` if the method is not specific to a trading pair.
    pub trading_pair_parameter: Option<&'static str>,

    /// The name of the parameter expected to contain the currency value (e.g., "currency").
    /// Used for validation and identifying the parameter.
    /// Set to `None` if the method is not specific to a currency.
    pub currency_parameter: Option<&'static str>,

    /// A list of parameter names expected to be provided either as query parameters (GET/DELETE)
    /// or as part of the request body (POST). This list does *not* include parameters
    /// used as placeholders in `path_segments`.
    pub parameters: &'static [&'static str],

    /// The name of the parameter representing the primary identifier (like order_id, trade_id)
    /// if it's used as a placeholder in the `path_segments`.
    /// Set to `None` if no such ID parameter exists in the path.
    pub id_parameter: Option<&'static str>,
}

/// Initializes and provides a static mapping of all supported Bitcoin.de API v4 methods to their configuration settings.
///
/// This function creates a comprehensive HashMap that defines the configuration for each API endpoint
/// supported by the Bitcoin.de API v4 SDK. Each entry maps a method name constant (e.g., `METHOD_SHOW_RATES`)
/// to its corresponding `MethodSetting` struct, which contains all the information needed to construct
/// valid API requests.
///
/// The configuration includes:
/// - HTTP method (GET, POST, DELETE)
/// - URL path structure with placeholders
/// - Parameter requirements and validation rules
/// - Identification of special parameters (trading pair, currency, ID)
///
/// This static initialization ensures that all API method configurations are defined once and
/// consistently used throughout the SDK.
///
/// # Returns
/// A `HashMap` where keys are method name constants and values are `MethodSetting` structs
/// containing the complete configuration for each API endpoint.
pub static METHOD_SETTINGS: LazyLock<HashMap<&'static str, MethodSetting>> = LazyLock::new(|| {
    // --- FIX: Import constants from the module defined ABOVE in this file ---
    use constants::*;

    let mut settings = HashMap::new();

    // --- Orders ---

    // GET /v4/:trading_pair/orderbook
    settings.insert(METHOD_SHOW_ORDERBOOK, MethodSetting {
        http_method: HTTP_METHOD_GET,
        path_segments: & [":trading_pair", "orderbook"], // :trading_pair is a path placeholder
        trading_pair_parameter: Some(SHOW_ORDERBOOK_PARAMETER_TRADING_PAIR), // Identifies the trading_pair param name
        currency_parameter: None,
        // These are the expected *query* parameters for GET
        parameters: &[
            SHOW_ORDERBOOK_PARAMETER_TYPE, // 'type'
            // Add other optional query params from docs if needed for validation later
            // "amount_currency_to_trade", "price", "order_requirements_fullfilled", etc.
        ],
        id_parameter: None,
    });

    // GET /v4/:trading_pair/orders/public/details/:order_id
    settings.insert(METHOD_SHOW_ORDER_DETAILS, MethodSetting {
        http_method: HTTP_METHOD_GET,
        path_segments: & [":trading_pair", "orders", "public", "details", ":order_id"], // Both :trading_pair and :order_id are path placeholders
        trading_pair_parameter: Some(SHOW_ORDER_DETAILS_PARAMETER_TRADING_PAIR),
        currency_parameter: None,
        parameters: &[], // No query or body parameters expected
        id_parameter: Some(SHOW_ORDER_DETAILS_PARAMETER_ORDER_ID), // Identifies the order_id param name used in path
    });

    // POST /v4/:trading_pair/orders
    settings.insert(METHOD_CREATE_ORDER, MethodSetting {
        http_method: HTTP_METHOD_POST,
        path_segments: & [":trading_pair", "orders"], // :trading_pair is a path placeholder
        trading_pair_parameter: Some(CREATE_ORDER_PARAMETER_TRADING_PAIR),
        currency_parameter: None,
        // These are the expected *body* parameters for POST
        parameters: &[
            CREATE_ORDER_PARAMETER_TYPE, // 'type'
            CREATE_ORDER_PARAMETER_MAX_AMOUNT, // 'max_amount_currency_to_trade' - Use constant
            CREATE_ORDER_PARAMETER_PRICE, // 'price'
            // Add other optional body params from docs if needed for validation later
            // "min_amount_currency_to_trade", "end_datetime", etc.
        ],
        id_parameter: None,
    });

    // DELETE /v4/:trading_pair/orders/:order_id
    settings.insert(METHOD_DELETE_ORDER, MethodSetting {
        http_method: HTTP_METHOD_DELETE,
        path_segments: & [":trading_pair", "orders", ":order_id"], // Both :trading_pair and :order_id are path placeholders
        trading_pair_parameter: Some(DELETE_ORDER_PARAMETER_TRADING_PAIR),
        currency_parameter: None,
        parameters: &[], // No query or body parameters expected
        id_parameter: Some(DELETE_ORDER_PARAMETER_ORDER_ID), // Identifies the order_id param name used in path
    });

    // GET /v4/orders (Show My Orders - All Pairs)
    // GET /v4/:trading_pair/orders (Show My Orders - Specific Pair)
    // Note: The SDK currently handles only one definition. We define the specific-pair one.
    // The calling code (`do_request`) needs logic to handle the case where trading_pair is None for the general endpoint.
    // Alternatively, define two separate method constants and entries if needed.
    settings.insert(METHOD_SHOW_MY_ORDERS, MethodSetting {
        http_method: HTTP_METHOD_GET,
        // This definition assumes a trading_pair is provided.
        // If no trading_pair is given, the path should just be "orders".
        // The build_url_path logic needs to handle optional path segments or have separate method definitions.
        // For now, defining it like the specific pair endpoint:
        path_segments: & [":trading_pair", "orders"], // Assumes specific pair for this definition
        trading_pair_parameter: Some(SHOW_MY_ORDERS_PARAMETER_TRADING_PAIR), // Parameter name if provided - Use constant
        currency_parameter: None,
        // Query parameters like 'type', 'state', 'date_start', 'date_end', 'page'
        parameters: &[
            // Add expected query params here if needed for validation
        ],
        id_parameter: None,
    });

    // GET /v4/:trading_pair/orders/:order_id (Show My Order Details)
    settings.insert(METHOD_SHOW_MY_ORDER_DETAILS, MethodSetting {
        http_method: HTTP_METHOD_GET,
        path_segments: & [":trading_pair", "orders", ":order_id"], // Path placeholders
        trading_pair_parameter: Some(SHOW_MY_ORDER_DETAILS_PARAMETER_TRADING_PAIR),
        currency_parameter: None,
        parameters: &[], // No query or body parameters
        id_parameter: Some(SHOW_MY_ORDER_DETAILS_PARAMETER_ORDER_ID), // Identifies order_id path param - Use constant
    });

    // --- Trades ---

    // POST /v4/:trading_pair/trades/:order_id
    settings.insert(METHOD_EXECUTE_TRADE, MethodSetting {
        http_method: HTTP_METHOD_POST,
        path_segments: & [":trading_pair", "trades", ":order_id"], // Path placeholders
        trading_pair_parameter: Some(EXECUTE_TRADE_PARAMETER_TRADING_PAIR),
        currency_parameter: None,
        // Expected *body* parameters for POST
        parameters: &[
            EXECUTE_TRADE_PARAMETER_TYPE, // 'type'
            EXECUTE_TRADE_PARAMETER_AMOUNT_CURRENCY_TO_TRADE, // 'amount_currency_to_trade'
            // Add optional 'payment_option' if needed
        ],
        id_parameter: Some(EXECUTE_TRADE_PARAMETER_ORDER_ID), // Identifies order_id path param
    });

    // GET /v4/trades (Show My Trades - All Pairs)
    // GET /v4/:trading_pair/trades (Show My Trades - Specific Pair)
    // Similar to showMyOrders, defining the specific-pair version here.
    settings.insert(METHOD_SHOW_MY_TRADES, MethodSetting {
        http_method: HTTP_METHOD_GET,
        path_segments: & [":trading_pair", "trades"], // Assumes specific pair for this definition
        trading_pair_parameter: Some(SHOW_MY_TRADES_PARAMETER_TRADING_PAIR), // Parameter name if provided - Use constant
        currency_parameter: None,
        // Query parameters like 'type', 'state', 'date_start', 'date_end', 'page', etc.
        parameters: &[
            // Add expected query params here if needed for validation
        ],
        id_parameter: None,
    });

    // --- Rest of the settings definitions follow the same pattern ---
    // --- Make sure to use the constants defined in the constants module ---

    // GET /v4/:trading_pair/trades/:trade_id
    settings.insert(METHOD_SHOW_MY_TRADE_DETAILS, MethodSetting {
        http_method: HTTP_METHOD_GET,
        path_segments: & [":trading_pair", "trades", ":trade_id"], // Path placeholders
        trading_pair_parameter: Some(SHOW_MY_TRADE_DETAILS_PARAMETER_TRADING_PAIR),
        currency_parameter: None,
        parameters: &[], // No query or body parameters
        id_parameter: Some(SHOW_MY_TRADE_DETAILS_PARAMETER_TRADE_ID), // Identifies trade_id path param
    });

    // POST /v4/:trading_pair/trades/:trade_id/mark_trade_as_paid
    settings.insert(METHOD_MARK_TRADE_AS_PAID, MethodSetting {
        http_method: HTTP_METHOD_POST,
        path_segments: & [":trading_pair", "trades", ":trade_id", "mark_trade_as_paid"], // Includes literal last segment
        trading_pair_parameter: Some(MARK_TRADE_AS_PAID_PARAMETER_TRADING_PAIR),
        currency_parameter: None,
        // Expected *body* parameter for POST
        parameters: &[MARK_TRADE_AS_PAID_PARAMETER_VOLUME_CURRENCY_TO_PAY_AFTER_FEE],
        id_parameter: Some(MARK_TRADE_AS_PAID_PARAMETER_TRADE_ID), // Identifies trade_id path param
    });

    // POST /v4/:trading_pair/trades/:trade_id/mark_trade_as_payment_received
    settings.insert(METHOD_MARK_TRADE_AS_PAYMENT_RECEIVED, MethodSetting {
        http_method: HTTP_METHOD_POST,
        path_segments: & [":trading_pair", "trades", ":trade_id", "mark_trade_as_payment_received"], // Includes literal last segment
        trading_pair_parameter: Some(MARK_TRADE_AS_PAYMENT_RECEIVED_PARAMETER_TRADING_PAIR),
        currency_parameter: None,
        // Expected *body* parameters for POST
        parameters: &[
            MARK_TRADE_AS_PAYMENT_RECEIVED_PARAMETER_VOLUME_CURRENCY_TO_PAY_AFTER_FEE,
            MARK_TRADE_AS_PAYMENT_RECEIVED_PARAMETER_RATING,
            MARK_TRADE_AS_PAYMENT_RECEIVED_PARAMETER_IS_PAID_FROM_CORRECT_BANK_ACCOUNT,
        ],
        id_parameter: Some(MARK_TRADE_AS_PAYMENT_RECEIVED_PARAMETER_TRADE_ID), // Identifies trade_id path param
    });

    // POST /v4/:trading_pair/trades/:trade_id/add_trade_rating
    settings.insert(METHOD_ADD_TRADE_RATING, MethodSetting {
        http_method: HTTP_METHOD_POST,
        path_segments: & [":trading_pair", "trades", ":trade_id", "add_trade_rating"], // Includes literal last segment
        trading_pair_parameter: Some(ADD_TRADE_RATING_PARAMETER_TRADING_PAIR),
        currency_parameter: None,
        // Expected *body* parameter for POST
        parameters: &[ADD_TRADE_RATING_PARAMETER_RATING],
        id_parameter: Some(ADD_TRADE_RATING_PARAMETER_TRADE_ID), // Identifies trade_id path param
    });

    // --- Account ---

    // GET /v4/account
    settings.insert(METHOD_SHOW_ACCOUNT_INFO, MethodSetting {
        http_method: HTTP_METHOD_GET,
        path_segments: & ["account"], // Simple literal path
        trading_pair_parameter: None,
        currency_parameter: None,
        parameters: &[], // No parameters
        id_parameter: None,
    });

    // GET /v4/:currency/account/ledger
    settings.insert(METHOD_SHOW_ACCOUNT_LEDGER, MethodSetting {
        http_method: HTTP_METHOD_GET,
        path_segments: & [":currency", "account", "ledger"], // :currency is path placeholder
        trading_pair_parameter: None,
        currency_parameter: Some(SHOW_ACCOUNT_LEDGER_PARAMETER_CURRENCY), // Identifies currency param name
        // Query parameters like 'type', 'datetime_start', 'datetime_end', 'page'
        parameters: &[
            // Add expected query params here if needed for validation
        ],
        id_parameter: None,
    });

    // GET /v4/permissions
    settings.insert(METHOD_SHOW_PERMISSIONS, MethodSetting {
        http_method: HTTP_METHOD_GET,
        path_segments: & ["permissions"], // Simple literal path
        trading_pair_parameter: None,
        currency_parameter: None,
        parameters: &[], // No parameters
        id_parameter: None,
    });

    // --- Withdrawals ---

    // POST /v4/:currency/withdrawals
    settings.insert(METHOD_CREATE_WITHDRAWAL, MethodSetting {
        http_method: HTTP_METHOD_POST,
        path_segments: & [":currency", "withdrawals"], // :currency is path placeholder
        trading_pair_parameter: None,
        currency_parameter: Some(CREATE_WITHDRAWAL_PARAMETER_CURRENCY), // Identifies currency param name
        // Expected *body* parameters for POST
        parameters: &[
            CREATE_WITHDRAWAL_PARAMETER_AMOUNT, // 'amount'
            CREATE_WITHDRAWAL_PARAMETER_ADDRESS, // 'address'
            CREATE_WITHDRAWAL_PARAMETER_NETWORK_FEE, // 'network_fee'
            // Add optional 'recipient_purpose', 'comment' if needed
        ],
        id_parameter: None,
    });

    // DELETE /v4/:currency/withdrawals/:withdrawal_id
    settings.insert(METHOD_DELETE_WITHDRAWAL, MethodSetting {
        http_method: HTTP_METHOD_DELETE,
        path_segments: & [":currency", "withdrawals", ":withdrawal_id"], // Path placeholders
        trading_pair_parameter: None,
        currency_parameter: Some(DELETE_WITHDRAWAL_PARAMETER_CURRENCY), // Identifies currency param name
        parameters: &[], // No query or body parameters
        id_parameter: Some(DELETE_WITHDRAWAL_PARAMETER_WITHDRAWAL_ID), // Identifies withdrawal_id path param
    });

    // GET /v4/:currency/withdrawals/:withdrawal_id
    settings.insert(METHOD_SHOW_WITHDRAWAL, MethodSetting {
        http_method: HTTP_METHOD_GET,
        path_segments: & [":currency", "withdrawals", ":withdrawal_id"], // Path placeholders
        trading_pair_parameter: None,
        currency_parameter: Some(SHOW_WITHDRAWAL_PARAMETER_CURRENCY), // Identifies currency param name
        parameters: &[], // No query or body parameters
        id_parameter: Some(SHOW_WITHDRAWAL_PARAMETER_WITHDRAWAL_ID), // Identifies withdrawal_id path param
    });

    // GET /v4/:currency/withdrawals/min_network_fee
    settings.insert(METHOD_SHOW_WITHDRAWAL_MIN_NETWORK_FEE, MethodSetting {
        http_method: HTTP_METHOD_GET,
        path_segments: & [":currency", "withdrawals", "min_network_fee"], // Includes literal last segment
        trading_pair_parameter: None,
        currency_parameter: Some(SHOW_WITHDRAWAL_PARAMETER_MIN_NETWORK_FEE_CURRENCY), // Identifies currency param name
        parameters: &[], // No query or body parameters
        id_parameter: None,
    });

    // GET /v4/:currency/withdrawals
    settings.insert(METHOD_SHOW_WITHDRAWALS, MethodSetting {
        http_method: HTTP_METHOD_GET,
        path_segments: & [":currency", "withdrawals"], // :currency is path placeholder
        trading_pair_parameter: None,
        currency_parameter: Some(SHOW_WITHDRAWALS_PARAMETER_CURRENCY), // Identifies currency param name
        // Query parameters like 'address', 'page'
        parameters: &[
            // Add expected query params here if needed for validation
        ],
        id_parameter: None,
    });

    // GET /v4/:currency/outgoing_address
    settings.insert(METHOD_SHOW_OUTGOING_ADDRESSES, MethodSetting {
        http_method: HTTP_METHOD_GET,
        path_segments: & [":currency", "outgoing_address"], // :currency is path placeholder
        trading_pair_parameter: None,
        currency_parameter: Some(SHOW_OUTGOING_ADDRESS_PARAMETER_CURRENCY), // Identifies currency param name
        // Query parameters like 'page'
        parameters: &[
            // Add expected query params here if needed for validation
        ],
        id_parameter: None,
    });

    // --- Deposits ---

    // POST /v4/:currency/deposits/new_address
    settings.insert(METHOD_REQUEST_DEPOSIT_ADDRESS, MethodSetting {
        http_method: HTTP_METHOD_POST,
        path_segments: & [":currency", "deposits", "new_address"], // Includes literal last segment
        trading_pair_parameter: None,
        currency_parameter: Some(REQUEST_DEPOSIT_ADDRESS_PARAMETER_CURRENCY), // Identifies currency param name
        // Optional *body* parameter 'comment'
        parameters: &[
            // Add 'comment' if needed for validation
        ],
        id_parameter: None,
    });

    // GET /v4/:currency/deposits/:deposit_id
    settings.insert(METHOD_SHOW_DEPOSIT, MethodSetting {
        http_method: HTTP_METHOD_GET,
        path_segments: & [":currency", "deposits", ":deposit_id"], // Path placeholders
        trading_pair_parameter: None,
        currency_parameter: Some(SHOW_DEPOSIT_PARAMETER_CURRENCY), // Identifies currency param name
        parameters: &[], // No query or body parameters
        id_parameter: Some(SHOW_DEPOSIT_PARAMETER_DEPOSIT_ID), // Identifies deposit_id path param
    });

    // GET /v4/:currency/deposits
    settings.insert(METHOD_SHOW_DEPOSITS, MethodSetting {
        http_method: HTTP_METHOD_GET,
        path_segments: & [":currency", "deposits"], // :currency is path placeholder
        trading_pair_parameter: None,
        currency_parameter: Some(SHOW_DEPOSITS_PARAMETER_CURRENCY), // Identifies currency param name
        // Query parameters like 'address', 'page'
        parameters: &[
            // Add expected query params here if needed for validation
        ],
        id_parameter: None,
    });

    // --- Misc ---

    // GET /v4/:trading_pair/orderbook/compact
    settings.insert(METHOD_SHOW_ORDERBOOK_COMPACT, MethodSetting {
        http_method: HTTP_METHOD_GET,
        path_segments: & [":trading_pair", "orderbook", "compact"], // Includes literal last segment
        trading_pair_parameter: Some(SHOW_ORDER_BOOK_COMPACT_PARAMETER_TRADING_PAIR), // Identifies trading_pair param name
        currency_parameter: None,
        parameters: &[], // No query or body parameters
        id_parameter: None,
    });

    // GET /v4/:trading_pair/trades/history
    settings.insert(METHOD_SHOW_PUBLIC_TRADE_HISTORY, MethodSetting {
        http_method: HTTP_METHOD_GET,
        path_segments: & [":trading_pair", "trades", "history"], // Includes literal last segment
        trading_pair_parameter: Some(SHOW_PUBLIC_TRADE_HISTORY_PARAMETER_TRADING_PAIR), // Identifies trading_pair param name
        currency_parameter: None,
        // Query parameter 'since_tid'
        parameters: &[
            // Add 'since_tid' if needed for validation
        ],
        id_parameter: None,
    });

    // GET /v4/:trading_pair/rates
    settings.insert(METHOD_SHOW_RATES, MethodSetting {
        http_method: HTTP_METHOD_GET,
        path_segments: & [":trading_pair", "rates"], // :trading_pair is path placeholder
        trading_pair_parameter: Some(SHOW_RATES_PARAMETER_TRADING_PAIR), // Identifies trading_pair param name
        currency_parameter: None,
        parameters: &[], // No query or body parameters
        id_parameter: None,
    });

    // --- Crypto-to-Crypto (External Wallet) ---
    // These methods often involve currency and address parameters in the path or body

    // POST /v4/:currency/address_pool
    settings.insert(METHOD_ADD_TO_ADDRESS_POOL, MethodSetting {
        http_method: HTTP_METHOD_POST,
        path_segments: & [":currency", "address_pool"], // :currency is path placeholder
        trading_pair_parameter: None,
        currency_parameter: Some(ADD_TO_ADDRESS_POOL_PARAMETER_CURRENCY), // Identifies currency param name
        // Expected *body* parameter 'address'
        parameters: &[ADD_TO_ADDRESS_POOL_PARAMETER_ADDRESS],
        id_parameter: None,
    });

    // DELETE /v4/:currency/address_pool/:address
    // Note: The API doc uses :address in the path, but the description mentions id_parameter. Assuming :address is the ID here.
    settings.insert(METHOD_REMOVE_FROM_ADDRESS_POOL, MethodSetting {
        http_method: HTTP_METHOD_DELETE,
        path_segments: & [":currency", "address_pool", ":address"], // Path placeholders
        trading_pair_parameter: None,
        currency_parameter: Some(REMOVE_FROM_ADDRESS_POOL_PARAMETER_CURRENCY), // Identifies currency param name
        parameters: &[], // No query or body parameters
        id_parameter: Some(REMOVE_FROM_ADDRESS_POOL_PARAMETER_ADDRESS), // Identifies address path param as the ID
    });

    // GET /v4/:currency/address_pool
    settings.insert(METHOD_LIST_ADDRESS_POOL, MethodSetting {
        http_method: HTTP_METHOD_GET,
        path_segments: & [":currency", "address_pool"], // :currency is path placeholder
        trading_pair_parameter: None,
        currency_parameter: Some(LIST_ADDRESS_POOL_PARAMETER_CURRENCY), // Identifies currency param name
        // Query parameter 'page'
        parameters: &[
            // Add 'page' if needed for validation
        ],
        id_parameter: None,
    });

    // POST /v4/:trading_pair/trades/:trade_id/mark_coins_as_transferred
    settings.insert(METHOD_MARK_COINS_AS_TRANSFERRED, MethodSetting {
        http_method: HTTP_METHOD_POST,
        path_segments: & [":trading_pair", "trades", ":trade_id", "mark_coins_as_transferred"], // Includes literal last segment
        trading_pair_parameter: Some(MARK_COINS_AS_TRANSFERRED_PARAMETER_TRADING_PAIR),
        currency_parameter: None,
        // Expected *body* parameter
        parameters: &[MARK_COINS_AS_TRANSFERRED_PARAMETER_AMOUNT_CURRENCY_TO_TRADE_AFTER_FEE],
        id_parameter: Some(MARK_COINS_AS_TRANSFERRED_PARAMETER_TRADE_ID), // Identifies trade_id path param
    });

    // POST /v4/:trading_pair/trades/:trade_id/mark_coins_as_received
    settings.insert(METHOD_MARK_COINS_AS_RECEIVED, MethodSetting {
        http_method: HTTP_METHOD_POST,
        path_segments: & [":trading_pair", "trades", ":trade_id", "mark_coins_as_received"], // Includes literal last segment
        trading_pair_parameter: Some(MARK_COINS_AS_RECEIVED_PARAMETER_TRADING_PAIR),
        currency_parameter: None,
        // Expected *body* parameters
        parameters: &[
            MARK_COINS_AS_RECEIVED_PARAMETER_AMOUNT_CURRENCY_TO_TRADE_AFTER_FEE,
            MARK_COINS_AS_RECEIVED_PARAMETER_RATING,
        ],
        id_parameter: Some(MARK_COINS_AS_RECEIVED_PARAMETER_TRADE_ID), // Identifies trade_id path param
    });

    // Return the populated HashMap
    settings
});
