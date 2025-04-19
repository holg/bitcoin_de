// bitcoin_de_trading_api_sdk_v4/responses/misc.rs
use serde::Deserialize;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};

// Import common structs from the parent module (mod.rs)
use super::{PageDetails, ApiErrorDetail};


// --- Misc Response Structs ---

/// Represents a compact order (bid or ask) in the compact orderbook.
/// Based on the "Bids" and "Asks" tables for showOrderbookCompact.
#[derive(Debug, Deserialize)]
// #[serde(rename_all = "snake_case")] // Apply snake_case if needed
pub struct CompactOrder {
    #[serde(with = "rust_decimal::serde::str")] // Numbers as strings in examples
    pub price: Decimal, // Float -> Decimal
    // Example JSON has both "amount" and "amount_currency_to_trade".
    // The table only lists "amount_currency_to_trade".
    #[serde(rename = "amount_currency_to_trade")] // Use table name
    #[serde(with = "rust_decimal::serde::str")] // Numbers as strings in examples
    pub amount_currency_to_trade: Decimal, // Float -> Decimal (example shows "amount")
}

/// Represents the bids and asks lists in the compact orderbook.
/// Based on the "Orders" table for showOrderbookCompact.
#[derive(Debug, Deserialize)]
pub struct CompactOrderbook {
    pub bids: Vec<CompactOrder>, // Array of CompactOrder
    pub asks: Vec<CompactOrder>, // Array of CompactOrder
}

/// Represents the successful response for `showOrderbookCompact`.
/// Based on the Success-Response example JSON structure.
#[derive(Debug, Deserialize)]
// #[serde(rename_all = "snake_case")] // Apply snake_case if needed
pub struct ShowOrderbookCompactResponse {
    #[serde(rename = "trading_pair")]
    pub trading_pair: String, // String
    pub orders: CompactOrderbook, // Nested object
    pub errors: Vec<ApiErrorDetail>, // Empty array on success
    pub credits: i32,
}


/// Represents a public trade history entry.
/// Based on the "Trade Informationen" table for showPublicTradeHistory.
#[derive(Debug, Deserialize)]
pub struct PublicTradeEntry {
    #[serde(with = "chrono::serde::ts_seconds")] // Unix timestamp
    pub date: DateTime<Utc>, // Integer (Unix timestamp) -> DateTime<Utc>
    #[serde(with = "rust_decimal::serde::str")] // Numbers as strings in examples
    pub price: Decimal, // Float -> Decimal
    #[serde(rename = "amount_currency_to_trade")] // Use the table name
    #[serde(with = "rust_decimal::serde::str")] // Numbers as strings in examples
    pub amount_currency_to_trade: Decimal, // Float -> Decimal (example shows "amount")
    pub tid: i64, // Integer -> i64 for potentially large IDs
}

/// Represents the successful response for `showPublicTradeHistory`.
/// Based on the Success-Response example JSON structure.
#[derive(Debug, Deserialize)]
// #[serde(rename_all = "snake_case")] // Apply snake_case if needed
pub struct ShowPublicTradeHistoryResponse {
    #[serde(rename = "trading_pair")]
    pub trading_pair: String, // String
    pub trades: Vec<PublicTradeEntry>, // Array of PublicTradeEntry
    pub errors: Vec<ApiErrorDetail>, // Empty array on success
    pub credits: i32,
}


/// Details within the `rates` object in `ShowRatesResponse`.
/// Based on the "Rates" table.
#[derive(Debug, Deserialize)]
// #[serde(rename_all = "snake_case")] // Apply snake_case if needed
pub struct RatesDetails {
    #[serde(rename = "rate_weighted")]
    #[serde(with = "rust_decimal::serde::str")]
    pub rate_weighted: Decimal, // String -> Decimal (string in example)
    #[serde(rename = "rate_weighted_3h")]
    #[serde(with = "rust_decimal::serde::str")]
    pub rate_weighted_3h: Decimal, // String -> Decimal (string in example)
    #[serde(rename = "rate_weighted_12h")]
    #[serde(with = "rust_decimal::serde::str")]
    pub rate_weighted_12h: Decimal, // String -> Decimal (string in example)
}

/// Represents the successful response for `showRates`.
/// Based on the Success-Response example JSON structure.
#[derive(Debug, Deserialize)]
// #[serde(rename_all = "snake_case")] // Apply snake_case if needed
pub struct ShowRatesResponse {
    #[serde(rename = "trading_pair")]
    pub trading_pair: String, // String
    pub rates: RatesDetails, // Nested object
    pub errors: Vec<ApiErrorDetail>, // Empty array on success
    pub credits: i32,
}


/// Represents outgoing address details (used for address pool as well).
/// Based on the structure shown in showOutgoingAddresses example.
#[derive(Debug, Deserialize)]
pub struct OutgoingAddressDetails {
    #[serde(rename = "address_id")]
    pub address_id: i64, // Integer (example 7411) -> i64
    #[serde(rename = "recipient_address")]
    pub recipient_address: String, // String
    #[serde(rename = "recipient_purpose")]
    pub recipient_purpose: Option<String>, // Optional String (example "123")
    pub comment: Option<String>, // Optional String (example "Mein Self-Hosted Wallet")
}

/// Represents the successful response for `showOutgoingAddresses` and `listAddressPool`.
/// Based on the Success-Response example JSON structure.
#[derive(Debug, Deserialize)]
pub struct ShowOutgoingAddressesResponse { // Also used for ListAddressPoolResponse
    #[serde(rename = "outgoing_address")]
    pub outgoing_address: Vec<OutgoingAddressDetails>, // Array
    pub page: PageDetails, // Nested object
    // Note: Example JSON shows "errors":{} - this is inconsistent with
    // other examples showing "errors":[] (array). Let's assume array based on "Error-Details" table.
    pub errors: Vec<ApiErrorDetail>,
    pub credits: i32,
}


// --- Simple Confirmation Responses ---
// Many methods (delete, mark, add/remove address pool, create/delete outgoing address)
// seem to return just errors and credits on success.

/// Represents a basic successful response containing only errors (empty on success) and credits.
#[derive(Debug, Deserialize)]
pub struct BasicSuccessResponse {
    pub errors: Vec<ApiErrorDetail>,
    pub credits: i32,
}

// Other methods that likely return BasicSuccessResponse:
// addToAddressPool
// removeFromAddressPool
// createOutgoingAddress
// deleteOutgoingAddress
// markCoinsAsTransferred
// markCoinsAsReceived