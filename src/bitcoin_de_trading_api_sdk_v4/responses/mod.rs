// bitcoin_de_trading_api_sdk_v4/responses/mod.rs
/// API response structures organized by category.
///
/// This module contains structs used to deserialize the JSON responses
/// received from the Bitcoin.de Trading API v4. They are organized
/// into sub-modules based on API endpoint categories.

// Declare the modules for each response category file in this directory
pub mod account;
pub mod deposits;
pub mod misc;
pub mod order; // User's preferred singular name
pub mod trades;
pub mod withdrawals;

// Re-export all structs from the sub-modules for easier access
// This allows `use super::responses::SomeResponseStruct;` or `use responses::*;` from the parent module
pub use account::*;
pub use deposits::*;
pub use misc::*;
pub use order::*;
pub use trades::*;
pub use withdrawals::*;
use serde::Deserialize;
use rust_decimal::Decimal;
use crate::bitcoin_de_trading_api_sdk_v4::errors::ApiErrorDetail;


// --- Common Nested Structs (defined here in responses/mod.rs) ---

/// Represents trading partner information.
/// Based on the "Trading Partner Information" table.
#[derive(Debug, Deserialize)]
// #[serde(rename_all = "snake_case")] // Apply snake_case if needed
pub struct TradingPartnerInformation {
    pub username: String,
    #[serde(rename = "is_kyc_full")]
    pub is_kyc_full: bool,
    #[serde(rename = "trust_level")]
    pub trust_level: String,
    #[serde(rename = "depositor")]
    pub depositor: Option<String>,
    pub iban: Option<String>,
    #[serde(rename = "bank_name")]
    pub bank_name: String,
    pub bic: String,
    #[serde(rename = "seat_of_bank")]
    pub seat_of_bank: Option<String>,
    #[serde(rename = "amount_trades")]
    pub amount_trades: i32,
    pub rating: i32,
}

/// Represents order requirements.
/// Based on the "Order Requirements" table.
#[derive(Debug, Deserialize)]
// #[serde(rename_all = "snake_case")] // Apply snake_case if needed
pub struct OrderRequirements {
    #[serde(rename = "min_trust_level")]
    pub min_trust_level: String,
    #[serde(rename = "only_kyc_full")]
    pub only_kyc_full: Option<bool>,
    #[serde(rename = "seat_of_bank")]
    pub seat_of_bank: Option<Vec<String>>,
    #[serde(rename = "payment_option")]
    pub payment_option: Option<i32>,
}

/// Represents page details in paged responses.
/// Based on the "Page Details" table.
#[derive(Debug, Deserialize)]
pub struct PageDetails {
    pub current: i32,
    pub last: i32,
}

/// Represents amounts in currency for Ledger entries (before/after fee).
/// Based on "Currency to trade" and "Currency to pay" tables.
#[derive(Debug, Deserialize)]
pub struct CurrencyAmounts {
    pub currency: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub before_fee: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub after_fee: Decimal,
}