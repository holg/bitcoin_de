// bitcoin_de_trading_api_sdk_v4/responses/withdrawals.rs
use serde::Deserialize;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};

// Import common structs from the parent module (mod.rs)
use super::{PageDetails, ApiErrorDetail};


// --- Withdrawal Response Structs ---

/// Represents withdrawal details.
/// Based on the "Withdrawal" table.
#[derive(Debug, Deserialize)]
// #[serde(rename_all = "snake_case")] // Apply snake_case if needed
pub struct WithdrawalDetails {
    #[serde(rename = "withdrawal_id")]
    pub withdrawal_id: String, // String (example shows String "123")
    pub address: String, // String
    #[serde(rename = "recipient_purpose")]
    pub recipient_purpose: Option<String>, // Optional String
    #[serde(with = "rust_decimal::serde::str")] // Numbers as strings in examples
    pub amount: Decimal, // Float -> Decimal
    #[serde(rename = "network_fee")]
    #[serde(with = "rust_decimal::serde::str")] // Numbers as strings in examples
    pub network_fee: Decimal, // Float -> Decimal
    pub comment: Option<String>, // Optional String (example shows "Bitpay", table says String)
    #[serde(rename = "created_at")]
    pub created_at: DateTime<Utc>, // String (RFC 3339) -> DateTime<Utc>
    pub state: i32, // Integer (Withdrawal-State-Values)
    #[serde(rename = "transferred_at")]
    pub transferred_at: Option<DateTime<Utc>>, // Optional DateTime
    pub txid: Option<String>, // Optional String (NULL in example)
}

/// Represents the successful response for `showWithdrawal`.
/// Note: The documentation shows the withdrawal details directly under a "withdrawal" key
/// in the top-level response for this specific endpoint, unlike `showWithdrawals`
/// which has an array.
#[derive(Debug, Deserialize)]
pub struct ShowWithdrawalResponse {
    pub withdrawal: WithdrawalDetails, // Nested object
    pub errors: Vec<ApiErrorDetail>, // Empty array on success
    pub credits: i32,
}

/// Represents the successful response for `showWithdrawals`.
/// Based on the Success-Response example JSON structure.
#[derive(Debug, Deserialize)]
pub struct ShowWithdrawalsResponse {
    pub withdrawals: Vec<WithdrawalDetails>, // Array of WithdrawalDetails
    pub page: PageDetails, // Nested object
    pub errors: Vec<ApiErrorDetail>, // Empty array on success
    pub credits: i32,
}

/// Represents the successful response for `createWithdrawal`.
/// Based on the Success-Response example, which shows a withdrawal_id.
#[derive(Debug, Deserialize)]
pub struct CreateWithdrawalResponse {
    #[serde(rename = "withdrawal_id")]
    pub withdrawal_id: i64, // Integer (example 123413) -> i64
    pub errors: Vec<ApiErrorDetail>, // Empty array on success
    pub credits: i32,
}

/// Represents the successful response for `deleteWithdrawal`.
/// Based on the Success-Response example, which is empty except errors/credits.
// Reuses the BasicSuccessResponse from the misc module.
pub type DeleteWithdrawalResponse = super::misc::BasicSuccessResponse;


/// Represents the successful response for `showWithdrawalMinNetworkFee`.
/// Based on the Success-Response example, which shows a min_network_fee string.
#[derive(Debug, Deserialize)]
pub struct ShowWithdrawalMinNetworkFeeResponse {
    #[serde(rename = "min_network_fee")]
    #[serde(with = "rust_decimal::serde::str")] // Numbers as strings in examples
    pub min_network_fee: Decimal, // String -> Decimal
    pub errors: Vec<ApiErrorDetail>, // Empty array on success
    pub credits: i32,
}