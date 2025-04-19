// bitcoin_de_trading_api_sdk_v4/responses/order.rs
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};

// Import common structs from the parent responses module (responses/mod.rs)
// This assumes TradingPartnerInformation, OrderRequirements, PageDetails,
// and ApiErrorDetail are defined or re-exported in responses/mod.rs
use super::{TradingPartnerInformation, OrderRequirements, PageDetails, ApiErrorDetail};


// --- Order Response Structs ---

/// Represents a single orderbook entry (bid or ask) from the public orderbook.
/// Based on the "Orders" table for showOrderbook.
#[derive(Debug, Deserialize, Serialize)]
// #[serde(rename_all = "snake_case")] // Apply snake_case if needed
pub struct OrderbookEntry {
    #[serde(rename = "order_id")]
    pub order_id: String, // String
    #[serde(rename = "is_external_wallet_order")]
    pub is_external_wallet_order: bool, // Boolean
    #[serde(rename = "trading_pair")]
    pub trading_pair: String, // String
    #[serde(rename = "type")] // "type" is a keyword, needs rename
    pub order_type: String, // String (buy/sell) - Renamed to avoid keyword conflict
    #[serde(rename = "max_amount_currency_to_trade")]
    #[serde(with = "rust_decimal::serde::str")]
    pub max_amount_currency_to_trade: Decimal, // Float -> Decimal (string in example)
    #[serde(rename = "min_amount_currency_to_trade")]
    #[serde(with = "rust_decimal::serde::str")]
    pub min_amount_currency_to_trade: Decimal, // Float -> Decimal (string in example)
    #[serde(with = "rust_decimal::serde::str")]
    pub price: Decimal, // Float -> Decimal (string in example)
    #[serde(rename = "max_volume_currency_to_pay")]
    #[serde(with = "rust_decimal::serde::str")]
    pub max_volume_currency_to_pay: Decimal, // Float -> Decimal (string in example)
    #[serde(rename = "min_volume_currency_to_pay")]
    #[serde(with = "rust_decimal::serde::str")]
    pub min_volume_currency_to_pay: Decimal, // Float -> Decimal (string in example)
    #[serde(rename = "order_requirements_fullfilled")]
    pub order_requirements_fullfilled: bool, // Boolean
    #[serde(rename = "sepa_option")]
    pub sepa_option: i32, // Integer
    #[serde(rename = "trading_partner_information")]
    pub trading_partner_information: TradingPartnerInformation, // Nested object
    #[serde(rename = "order_requirements")]
    pub order_requirements: OrderRequirements, // Nested object
}


/// Represents the successful response for `showOrderbook`.
/// Based on the Success-Response example JSON structure.
#[derive(Debug, Deserialize, Serialize)]
pub struct ShowOrderbookResponse {
    pub orders: Vec<OrderbookEntry>, // Array of OrderbookEntry
    pub errors: Vec<ApiErrorDetail>, // Empty array on success
    pub credits: i32,
}

/// Represents details of a single one of your orders.
/// Based on the "Order-Details" table for showMyOrders and showMyOrderDetails.
#[derive(Debug, Deserialize, Serialize)]
// #[serde(rename_all = "snake_case")] // Apply snake_case if needed
pub struct MyOrderDetails {
    #[serde(rename = "order_id")]
    pub order_id: String, // String
    #[serde(rename = "trading_pair")]
    pub trading_pair: String, // String
    #[serde(rename = "is_external_wallet_order")]
    pub is_external_wallet_order: bool, // Boolean
    #[serde(rename = "type")] // "type" is a keyword
    pub order_type: String, // String (buy/sell) - Renamed
    #[serde(rename = "max_amount_currency_to_trade")]
    #[serde(with = "rust_decimal::serde::str")]
    pub max_amount_currency_to_trade: Decimal, // Float -> Decimal (string in example)
    #[serde(rename = "min_amount_currency_to_trade")]
    #[serde(with = "rust_decimal::serde::str")]
    pub min_amount_currency_to_trade: Decimal, // Float -> Decimal (string in example)
    #[serde(with = "rust_decimal::serde::str")]
    pub price: Decimal, // Float -> Decimal (string in example)
    #[serde(rename = "max_volume_currency_to_pay")]
    #[serde(with = "rust_decimal::serde::str")]
    pub max_volume_currency_to_pay: Decimal, // Float -> Decimal (string in example)
    #[serde(rename = "min_volume_currency_to_pay")]
    #[serde(with = "rust_decimal::serde::str")]
    pub min_volume_currency_to_pay: Decimal, // Float -> Decimal (string in example)
    #[serde(rename = "end_datetime")]
    // Use chrono for datetime, format based on docs (RFC 3339)
    pub end_datetime: Option<DateTime<Utc>>, // Optional DateTime
    #[serde(rename = "new_order_for_remaining_amount")]
    pub new_order_for_remaining_amount: bool, // Boolean
    pub state: i32, // Integer (Order-State-Values)
    #[serde(rename = "order_requirements")]
    // Note: showMyOrders example JSON includes order_requirements directly in the order object
    pub order_requirements: OrderRequirements, // Nested object
    #[serde(rename = "sepa_option")]
    pub sepa_option: i32, // Integer
    #[serde(rename = "created_at")]
    // Use chrono for datetime, format based on docs (RFC 3339)
    pub created_at: DateTime<Utc>,
    // Note: showMyOrders example JSON includes trading_partner_information directly
    // in the order object, but the table "Order-Details" doesn't list it.
    // showMyOrderDetails table also doesn't list it, but its example JSON does!
    // Let's include it as optional based on the example JSON.
    #[serde(rename = "trading_partner_information")]
    pub trading_partner_information: Option<TradingPartnerInformation>, // Optional Nested object
}


/// Represents the successful response for `showMyOrders`.
/// Based on the Success-Response example JSON structure.
#[derive(Debug, Deserialize, Serialize)]
pub struct ShowMyOrdersResponse {
    pub orders: Vec<MyOrderDetails>, // Array of MyOrderDetails
    pub page: PageDetails, // Nested object
    pub errors: Vec<ApiErrorDetail>, // Empty array on success
    pub credits: i32,
}


/// Represents the successful response for `showMyOrderDetails`.
/// Based on the Success-Response example JSON structure.
// This struct uses MyOrderDetails for its 'order' field.
#[derive(Debug, Deserialize, Serialize)]
pub struct ShowOrderDetailsResponse {
    // The example JSON has the order details directly under "order".
    #[serde(rename = "order")] // Field name in the JSON response
    pub order_details: MyOrderDetails, // Nested MyOrderDetails object (renamed field)
    pub errors: Vec<ApiErrorDetail>, // Empty array on success
    pub credits: i32,
}

/// Represents the successful response for `createOrder`.
/// Based on the Success-Response example JSON structure.
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateOrderResponse {
    #[serde(rename = "order_id")]
    pub order_id: String, // String (example "A1234BC")
    pub errors: Vec<ApiErrorDetail>, // Empty array on success
    pub credits: i32,
}

/// Represents the successful response for `deleteOrder`.
/// Based on the Success-Response example JSON structure.
#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteOrderResponse {
    pub errors: Vec<ApiErrorDetail>, // Empty array on success
    pub credits: i32,
}