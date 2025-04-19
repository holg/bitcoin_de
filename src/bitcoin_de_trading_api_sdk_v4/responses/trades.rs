// bitcoin_de_trading_api_sdk_v4/responses/trades.rs
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

// Import common structs from the parent module (mod.rs) and the BasicSuccessResponse
use super::{TradingPartnerInformation, CurrencyAmounts, PageDetails, ApiErrorDetail, misc::BasicSuccessResponse};


// --- Trade Response Structs ---

/// Represents details of a single one of your trades.
/// Based on the "Trade-Details" table for showMyTrades and showMyTradeDetails.
#[derive(Debug, Deserialize, Serialize)]
// #[serde(rename_all = "snake_case")] // Apply snake_case if needed
pub struct MyTradeDetails {
    #[serde(rename = "trade_id")]
    pub trade_id: String, // String
    #[serde(rename = "is_external_wallet_trade")]
    pub is_external_wallet_trade: bool, // Boolean
    #[serde(rename = "trading_pair")]
    pub trading_pair: String, // String
    #[serde(rename = "type")] // "type" is a keyword
    pub trade_type: String, // String (buy/sell) - Renamed
    #[serde(rename = "amount_currency_to_trade")]
    #[serde(with = "rust_decimal::serde::str")]
    pub amount_currency_to_trade: Decimal, // Float -> Decimal (string in example)
    #[serde(with = "rust_decimal::serde::str")]
    pub price: Decimal, // Float -> Decimal (string in example)
    #[serde(rename = "volume_currency_to_pay")]
    #[serde(with = "rust_decimal::serde::str")]
    pub volume_currency_to_pay: Decimal, // Float -> Decimal (string in example)
    #[serde(rename = "amount_currency_to_trade_after_fee")]
    #[serde(with = "rust_decimal::serde::str")]
    pub amount_currency_to_trade_after_fee: Decimal, // Float -> Decimal (string in example)
    #[serde(rename = "volume_currency_to_pay_after_fee")]
    #[serde(with = "rust_decimal::serde::str")]
    pub volume_currency_to_pay_after_fee: Decimal, // Float -> Decimal (string in example)
    #[serde(rename = "fee_currency_to_pay")]
    #[serde(with = "rust_decimal::serde::str")]
    pub fee_currency_to_pay: Decimal, // Float -> Decimal (string in example)
    #[serde(rename = "fee_currency_to_trade")]
    #[serde(with = "rust_decimal::serde::str")]
    pub fee_currency_to_trade: Decimal, // Float -> Decimal (string in example)
    #[serde(rename = "new_order_id_for_remaining_amount")]
    pub new_order_id_for_remaining_amount: Option<String>, // Optional String
    pub state: i32, // Integer (Trade-State-Values)
    #[serde(rename = "is_trade_marked_as_paid")]
    pub is_trade_marked_as_paid: Option<bool>, // Optional Boolean (only in showMyTradeDetails example)
    #[serde(rename = "trade_marked_as_paid_at")]
    pub trade_marked_as_paid_at: Option<DateTime<Utc>>, // Optional DateTime
    #[serde(rename = "my_rating_for_trading_partner")]
    pub my_rating_for_trading_partner: Option<String>, // Optional String (pending, etc.)
    #[serde(rename = "trading_partner_information")]
    // Note: showMyTradeDetails example JSON includes trading_partner_information
    pub trading_partner_information: TradingPartnerInformation, // Nested object
    #[serde(rename = "created_at")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "successfully_finished_at")]
    pub successfully_finished_at: Option<DateTime<Utc>>, // Optional DateTime
    #[serde(rename = "cancelled_at")]
    pub cancelled_at: Option<DateTime<Utc>>, // Optional DateTime
    #[serde(rename = "payment_method")]
    pub payment_method: i32, // Integer (Payment method values)

    // Handle the nested currency details in the showAccountLedger trade details example
    // "primary_currency": {"currency_to_trade": { ... }}
    // "secondary_currency": {"currency_to_pay": { ... }}
    // This structure seems specific to account ledger trade details.
    #[serde(rename = "primary_currency")]
    pub primary_currency: Option<HashMap<String, CurrencyAmounts>>, // Example shows "currency_to_trade" as key
    #[serde(rename = "secondary_currency")]
    pub secondary_currency: Option<HashMap<String, CurrencyAmounts>>, // Example shows "currency_to_pay" as key

    // Handle the inconsistent "amount" and "volume" fields in the showMyTrades example
    // If these exist in the JSON AND have different meaning than the ..._currency fields,
    // you might need Option fields and serde aliases. But the documentation tables
    // only list the ..._currency fields as the standard ones. Let's stick to the table.
    // If the example JSON is correct and different, need to map:
    // #[serde(rename = "amount")]
    // pub amount_from_example: Option<Decimal>,
    // #[serde(rename = "volume")]
    // pub volume_from_example: Option<Decimal>,
}


/// Represents the successful response for `showMyTrades`.
/// Based on the Success-Response example JSON structure.
#[derive(Debug, Deserialize, Serialize)]
pub struct ShowMyTradesResponse {
    pub trades: Vec<MyTradeDetails>, // Array of MyTradeDetails
    pub page: PageDetails, // Nested object
    pub errors: Vec<ApiErrorDetail>, // Empty array on success
    pub credits: i32,
}


/// Represents the successful response for `showMyTradeDetails`.
/// Based on the Success-Response example JSON structure.
#[derive(Debug, Deserialize, Serialize)]
pub struct ShowMyTradeDetailsResponse {
    // The example JSON has the trade details directly under "trade".
    pub trade: MyTradeDetails, // Nested MyTradeDetails object
    pub errors: Vec<ApiErrorDetail>, // Empty array on success
    pub credits: i32,
}

// Simple confirmation responses that likely return only errors and credits

/// Represents the successful response for `executeTrade`.
/// Based on the Success-Response example, which shows an empty body except errors/credits.
/// Location header is also returned, but not part of the body for deserialization.
pub type ExecuteTradeResponse = BasicSuccessResponse;

/// Represents the successful response for `markTradeAsPaid`.
/// Based on the Success-Response example.
pub type MarkTradeAsPaidResponse = BasicSuccessResponse;

/// Represents the successful response for `markTradeAsPaymentReceived`.
/// Based on the Success-Response example.
pub type MarkTradeAsPaymentReceivedResponse = BasicSuccessResponse;

/// Represents the successful response for `addTradeRating`.
/// Based on the Success-Response example.
pub type AddTradeRatingResponse = BasicSuccessResponse;