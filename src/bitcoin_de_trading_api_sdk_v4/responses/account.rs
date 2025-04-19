// bitcoin_de_trading_api_sdk_v4/responses/account.rs
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

// Import common structs from the parent module (mod.rs)
use super::{CurrencyAmounts, PageDetails, ApiErrorDetail};


// --- Account Response Structs ---

/// Details within the `account_info` object in `ShowAccountInfoResponse`.
#[derive(Debug, Deserialize, Serialize)]
// #[serde(rename_all = "snake_case")] // Apply snake_case if needed
pub struct AccountInfoDetails {
    pub username: String,
    // The documentation and example JSON do not explicitly show 'registered_at' or 'trust_level'
    // directly in this 'account_info' block, although they are mentioned elsewhere.
    // Add fields here if you confirm they appear in the API response under 'account_info'.
    // pub registered_at: Option<DateTime<Utc>>, // Example: If API provides this field in RFC 3339
    // pub trust_level: Option<String>, // Example: If API provides this field
}

/// Represents a Fidor bank reservation for trading on Bitcoin.de.
/// which shall be outdated and obsolete, as there is no Fidor Bank anymore.
/// This struct contains information about the reserved funds at Fidor Bank
/// that can be used for trading on the Bitcoin.de platform, including the
/// total and available amounts, reservation timestamps, and allocation
/// across different cryptocurrencies.
#[derive(Debug, Deserialize, Serialize)]
// #[serde(rename_all = "snake_case")] // Apply snake_case if needed
pub struct FidorReservation {
    /// The total amount of money reserved at Fidor Bank for trading.
    /// Represented as a string in the API response and converted to Decimal.
    #[serde(with = "rust_decimal::serde::str")]
    pub total_amount: Decimal,

    /// The amount of money that is still available for trading from the reservation.
    /// Represented as a string in the API response and converted to Decimal.
    #[serde(with = "rust_decimal::serde::str")]
    pub available_amount: Decimal,

    /// The timestamp when the funds were reserved at Fidor Bank.
    /// Format follows RFC 3339 standard.
    #[serde(rename = "reserved_at")]
    pub reserved_at: DateTime<Utc>,

    /// The timestamp until which the reservation is valid.
    /// After this time, the reservation expires if not used.
    #[serde(rename = "valid_until")]
    pub valid_until: DateTime<Utc>,

    /// The allocation of the reserved funds across different cryptocurrencies.
    /// Keys are cryptocurrency codes (e.g., "btc"), and values contain allocation details.
    pub allocation: HashMap<String, FidorAllocation>,
}

/// Details within the currency allocation in `FidorReservation`.
/// Based on the "Prozentuale Aufteilung" and currency tables (BTC, BCH).
#[derive(Debug, Deserialize, Serialize)]
// #[serde(rename_all = "snake_case")] // Apply snake_case if needed
pub struct FidorAllocation {
    pub percent: i32, // String -> Integer (based on table, example uses int)
    #[serde(with = "rust_decimal::serde::str")]
    pub max_eur_volume: Decimal, // String -> Decimal
    #[serde(rename = "eur_volume_open_orders")]
    #[serde(with = "rust_decimal::serde::str")]
    pub eur_volume_open_orders: Decimal, // String -> Decimal
}


/// Details within the `encrypted_information` object.
/// Based on the "Encrypted-Information" table.
#[derive(Debug, Deserialize, Serialize)]
// #[serde(rename_all = "snake_case")] // Apply snake_case if needed
pub struct EncryptedInformation {
    #[serde(rename = "bic_short")]
    pub bic_short: Option<String>, // Optional String
    #[serde(rename = "bic_full")]
    pub bic_full: Option<String>, // Optional String
    pub uid: String, // String
}

/// Represents the successful response for `showAccountInfo`.
/// Based *strictly* on the user's provided successful JSON output structure:
/// {"data":{...},"errors":[],"credits":28}
#[derive(Debug, Deserialize, Serialize)]
pub struct ShowAccountInfoResponse {
    pub data: ShowAccountInfoData, // The main content is under a field named 'data'
    pub errors: Vec<ApiErrorDetail>, // Empty array on success
    pub credits: i32,
}

/// Represents the structure of the 'data' field in the `showAccountInfo` response JSON.
/// Based *strictly* on the user's provided successful JSON output structure:
/// {"balances":{...},"encrypted_information":{...}}
#[derive(Debug, Deserialize, Serialize)]
pub struct ShowAccountInfoData {
    pub balances: AccountBalances, // Contains the detailed crypto balances
    #[serde(rename = "encrypted_information")]
    pub encrypted_information: EncryptedInformation, // Nested struct
    // Note: The example JSON does NOT show 'account_info' or 'fidor_reservation' within 'data'.
    // They might be optional top-level fields, but are not part of the 'data' struct based on the JSON.
}

/// Represents the structure of the 'balances' field within the 'data' object
/// in the `showAccountInfo` response JSON.
/// Based *strictly* on the user's provided successful JSON output structure:
/// {"btc":{...},"bch":{...},...}
#[derive(Debug, Deserialize, Serialize)]
pub struct AccountBalances {
    // Based on the example JSON, this contains keys like "btc", "bch", etc.,
    // each mapping to a struct with "total_amount", "available_amount", "reserved_amount".
    #[serde(flatten)] // Flatten the map of currency tickers -> DetailedBalanceAmounts structs
    pub crypto_balances: HashMap<String, DetailedBalanceAmounts>,
    // The example JSON does NOT show fiat balances within this 'balances' object or at the top level,
    // despite the documentation mentioning them. If fiat balances appear elsewhere,
    // they'll need a separate struct/field.
}

/// Represents the detailed balance amounts for a single cryptocurrency.
/// Based on the structure of the objects nested under currency tickers in
/// the 'balances' field of the `showAccountInfo` response JSON.
#[derive(Debug, Deserialize, Serialize)]
pub struct DetailedBalanceAmounts {
    #[serde(rename = "total_amount")]
    #[serde(with = "rust_decimal::serde::str")]
    pub total_amount: Decimal,
    #[serde(rename = "available_amount")]
    #[serde(with = "rust_decimal::serde::str")]
    pub available_amount: Decimal,
    #[serde(rename = "reserved_amount")]
    #[serde(with = "rust_decimal::serde::str")]
    pub reserved_amount: Decimal,
}


/// Represents a ledger entry.
/// Based on the "Details zur Position" table for showAccountLedger.
#[derive(Debug, Deserialize, Serialize)]
// #[serde(rename_all = "snake_case")] // Apply snake_case if needed
pub struct LedgerEntry {
    #[serde(rename = "date")]
    pub date: DateTime<Utc>, // String (RFC 3339) -> DateTime<Utc>
    #[serde(rename = "type")] // "type" is a keyword
    pub entry_type: String, // String (all, buy, sell, etc.) - Renamed
    pub reference: String, // String (trade_id or txid)
    #[serde(rename = "trade")]
    pub trade_details: Option<LedgerTradeDetails>, // Optional Nested object
    #[serde(with = "rust_decimal::serde::str")] // Numbers as strings in examples
    pub cashflow: Decimal, // String -> Decimal
    #[serde(with = "rust_decimal::serde::str")] // Numbers as strings in examples
    pub balance: Decimal, // String -> Decimal
}

/// Represents trade details within a ledger entry.
/// Based on the "Tradedetails" table for showAccountLedger.
#[derive(Debug, Deserialize, Serialize)]
// #[serde(rename_all = "snake_case")] // Apply snake_case if needed
pub struct LedgerTradeDetails {
    #[serde(rename = "trade_id")]
    pub trade_id: String,
    #[serde(rename = "trading_pair")]
    pub trading_pair: String,
    #[serde(with = "rust_decimal::serde::str")] // Numbers as strings in examples
    pub price: Decimal,
    #[serde(rename = "is_external_wallet_trade")]
    pub is_external_wallet_trade: bool,
    #[serde(rename = "primary_currency")]
    // Example shows "currency_to_trade" as key
    pub primary_currency: HashMap<String, CurrencyAmounts>,
    #[serde(rename = "secondary_currency")]
    // Example shows "currency_to_pay" as key
    pub secondary_currency: HashMap<String, CurrencyAmounts>,
    // Note: The example JSON has "btc" and "euro" keys directly under trade for old entries?
    // Let's follow the table structure "primary_currency", "secondary_currency" as standard v4.
    // If you need to support old formats, you might need custom deserialization.
    // Example: "btc": {"before_fee":"0.55676560","after_fee":"0.55119794"}
    // #[serde(flatten)] // Attempt to flatten unknown keys into a map?
    // extra_currencies: HashMap<String, HashMap<String, Decimal>>,
}


/// Represents the successful response for `showAccountLedger`.
/// Based on the Success-Response example JSON structure.
#[derive(Debug, Deserialize, Serialize)]
pub struct ShowAccountLedgerResponse {
    #[serde(rename = "account_ledger")]
    pub account_ledger: Vec<LedgerEntry>,
    pub page: PageDetails,
    pub errors: Vec<ApiErrorDetail>,
    pub credits: i32,
}

/// Represents the successful response for `showPermissions`.
#[derive(Debug, Deserialize, Serialize)]
pub struct ShowPermissionsResponse {
    pub permissions: Vec<String>, // Array of Strings
    pub errors: Vec<ApiErrorDetail>,
    pub credits: i32,
}