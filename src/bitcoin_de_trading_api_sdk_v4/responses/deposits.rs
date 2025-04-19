// bitcoin_de_trading_api_sdk_v4/responses/deposits.rs
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};

// Import common structs from the parent module (mod.rs)
use super::{PageDetails, ApiErrorDetail};


// --- Deposit Response Structs ---

/// Represents deposit details.
/// Based on the "Deposit" table.
#[derive(Debug, Deserialize, Serialize)]
// #[serde(rename_all = "snake_case")] // Apply snake_case if needed
pub struct DepositDetails {
    #[serde(rename = "deposit_id")]
    pub deposit_id: i64, // Integer (example 123) -> i64
    pub address: String, // String
    #[serde(rename = "recipient_purpose")]
    pub recipient_purpose: Option<String>, // Optional String
    #[serde(with = "rust_decimal::serde::str")] // Numbers as strings in examples
    pub amount: Decimal, // Float -> Decimal
    pub state: i32, // Integer (Deposit-State-Values)
    pub txid: String, // String
    pub confirmations: i32, // Integer
    #[serde(rename = "created_at")]
    pub created_at: DateTime<Utc>, // String (RFC 3339) -> DateTime<Utc>
}

/// Represents the successful response for `showDeposit`.
/// Note: The documentation shows the deposit details directly under a "deposit" key
/// in the top-level response for this specific endpoint, unlike `showDeposits`
/// which has an array.
#[derive(Debug, Deserialize, Serialize)]
pub struct ShowDepositResponse {
    pub deposit: DepositDetails, // Nested object
    pub errors: Vec<ApiErrorDetail>, // Empty array on success
    pub credits: i32,
}

/// Represents the successful response for `showDeposits`.
/// Based on the Success-Response example JSON structure.
#[derive(Debug, Deserialize, Serialize)]
pub struct ShowDepositsResponse {
    // Note: Example JSON had nested "deposits": [...] within a top-level "deposits" key.
    // Following the table description and common patterns, assuming top-level contains the array.
    pub deposits: Vec<DepositDetails>, // Array of DepositDetails
    pub page: PageDetails, // Nested object
    pub errors: Vec<ApiErrorDetail>, // Empty array on success
    pub credits: i32,
}

/// Represents the successful response for `requestDepositAddress`.
/// Based on the Success-Response example JSON structure.
#[derive(Debug, Deserialize, Serialize)]
pub struct RequestDepositAddressResponse {
    pub address: String, // String
    #[serde(rename = "recipient_purpose")]
    pub recipient_purpose: Option<String>, // Optional String
    pub errors: Vec<ApiErrorDetail>, // Empty array on success
    pub credits: i32,
}