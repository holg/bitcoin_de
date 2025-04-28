# Rust Bitcoin.de Trading API Client (v4)

[![crates.io](https://img.shields.io/crates/v/bitcoin_de.svg?style=flat-square)](https://crates.io/crates/bitcoin_de) <!-- Placeholder: Update if published -->
[![docs.rs](https://img.shields.io/docsrs/bitcoin_de?style=flat-square)](https://docs.rs/bitcoin_de/) <!-- Placeholder: Update if published -->
[![Build Status](https://img.shields.io/github/actions/workflow/status/holg/bitcoin_de/rust.yml?branch=main&style=flat-square)](https://github.com/holg/bitcoin_de/actions) <!-- Assuming GH username/repo -->
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat-square)](https://opensource.org/licenses/MIT)

A Rust client library (SDK) for interacting with the [Bitcoin.de Trading API (Version 4)](https://www.bitcoin.de/de/api/tapi/v4/docu), with an optional Axum-based backend feature providing HTTP endpoints.

**Note:** While many core API methods are implemented in the SDK, comprehensive
# Bitcoin.de Trading API Client

A Rust client library and command-line tool for interacting with the Bitcoin.de Trading API v4.

## Overview

This project provides a comprehensive SDK for the Bitcoin.de cryptocurrency trading platform API. It allows developers to programmatically access trading functionality, account information, and market data through a clean, type-safe Rust interface.

## Features

- Complete implementation of the Bitcoin.de Trading API v4
- Support for all trading operations (view orderbook, create/delete orders, execute trades)
- Account management (view info, deposits, withdrawals)
- Secure authentication using HMAC-SHA256 signatures
- Comprehensive error handling with detailed error codes
- Command-line interface for quick operations
- Fully documented API methods and data structures

## Installation

### Prerequisites

- Rust 1.56 or later
- API credentials from Bitcoin.de

### From crates.io

```bash
cargo install bitcoin_de
```

### From Source

```bash
git clone https://github.com/holg/bitcoin_de.git
cd bitcoin_de
cargo build --release
```

## Configuration

On bitcoin.de you need to set up your API token and secret:
[https://www.bitcoin.de/de/userprofile/tapi](https://www.bitcoin.de/de/userprofile/tapi)

Create a `.env` file in the project root with your Bitcoin.de API credentials:

```dotenv
API_KEY=your_bitcoin_de_api_key
API_SECRET=your_bitcoin_de_api_secret
```

You can use the provided `.env.sample` as a template.  
Alternatively, you can pass the `--api-key` and `--api-secret` to the command line.

## Usage

### As a Library

```rust
// Example: Get Account Info and Orderbook using the Bitcoin.de Trading API v4

// Import necessary types from your crate
use bitcoin_de::bitcoin_de_trading_api_sdk_v4::TradingApiSdkV4;
// We don't need to import ApiCredentials anymore as TradingApiSdkV4::new takes strings directly
// use bitcoin_de::bitcoin_de_trading_api_sdk_v4::config::ApiCredentials; // Remove this import

// Import other types needed for the example
use std::collections::HashMap; // Needed for show_orderbook parameters
use bitcoin_de::bitcoin_de_trading_api_sdk_v4::method_settings::constants::SHOW_ORDERBOOK_PARAMETER_TYPE; // Need this constant for show_orderbook params
use bitcoin_de::bitcoin_de_trading_api_sdk_v4::responses::order::ShowOrderbookResponse; // Import the response struct type for show_orderbook
use bitcoin_de::bitcoin_de_trading_api_sdk_v4::responses::account::ShowAccountInfoResponse; // Import the response struct type for show_account_info
use bitcoin_de::bitcoin_de_trading_api_sdk_v4::errors::Error; // Import the custom Error type

// Add an async runtime setup for the example main function
// Using tokio as it's used by your default CLI
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // In a real application, get your API key and secret securely
    // from environment variables, a configuration file, etc.
    // For this example, replace with your actual credentials or environment variable loading logic.
    let api_key = "your_api_key".to_string(); // Replace with your API Key
    let api_secret = "your_api_secret".to_string(); // Replace with your API Secret

    // Create a new API client with your credentials
    // TradingApiSdkV4::new takes the key and secret directly as Strings
    let client = TradingApiSdkV4::new(api_key, api_secret);

    // --- Example 1: Get Account Info ---
    println!(">>> Calling showAccountInfo...");
    // Call the async SDK method and await its result
    let account_info_result: Result<ShowAccountInfoResponse, Error> = client.show_account_info().await;

    match account_info_result {
        Ok(response) => {
            println!("showAccountInfo successful Response: {:?}", response);
            // Access deserialized data, e.g.:
            // if let Some(account_details) = response.account_info_details {
            //     println!("Username: {}", account_details.username);
            // }
        }
        Err(e) => {
            eprintln!("Error getting account info: {}", e);
        }
    }

    // --- Example 2: Get the current orderbook for BTC/EUR (Buy side) ---
    println!("\n>>> Calling showOrderbook...");
    // show_orderbook takes the trading pair as a String and Option<HashMap> for other params
    let mut params = HashMap::new();
    // Add the 'type=buy' parameter to the HashMap
    params.insert(SHOW_ORDERBOOK_PARAMETER_TYPE, "buy".to_string());

    // Call the async SDK method and await its result
    // show_orderbook returns Result<ShowOrderbookResponse, Error>
    let orderbook_result: Result<ShowOrderbookResponse, Error> = client.show_orderbook("btceur".to_string(), Some(params)).await; // Pass trading pair as String and params

    match orderbook_result {
        Ok(response) => {
            println!("Orderbook: {:?}", response);
            // Access deserialized data, e.g.:
            // for order in response.orders {
            //     println!("{:?}", order);
            // }
        }
        Err(e) => {
            eprintln!("Error getting orderbook: {}", e);
        }
    }

    // Return Ok(()) for the async main function on success
    Ok(())
}
```

### As a Command-Line Tool


# View the BTC/EUR orderbook
```bash
bitcoin_de_trading_api_client show-orderbook --trading-pair btceur --type buy
```
# Create a buy order
```bash
bitcoin_de_trading_api_client create-order --trading-pair btceur --type buy --amount 0.1 --price 50000
```
# View your account information
```bash
bitcoin_de_trading_api_client show-account-info
```
# Show help
```bash
bitcoin_de_trading_api_client --help
```

## Error Handling

The library provides detailed error codes for troubleshooting API interactions. For example:

- `ERROR_CODE_ORDER_NOT_FOUND` (13): The requested order could not be found
- `ERROR_CODE_INSUFFICIENT_CREDITS` (22): The user has insufficient credits to complete the operation
- `ERROR_CODE_INVALID_TRADING_PAIR` (32): The trading pair specified is invalid

See the documentation for a complete list of error codes and their meanings.

## API Methods

The client supports all Bitcoin.de Trading API v4 methods, including:

- Market data: `show_orderbook`, `show_rates`, `show_public_trade_history`
- Order management: `create_order`, `delete_order`, `show_my_orders`
- Trading: `execute_trade`, `mark_trade_as_paid`, `mark_trade_as_payment_received`
- Account operations: `show_account_info`, `show_account_ledger`
- Deposits and withdrawals: `create_withdrawal`, `request_deposit_address`

## License

This project is licensed under the MIT License - see the `LICENSE` file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Disclaimer

This software is not affiliated with Bitcoin.de. Use at your own risk. Always verify important operations before executing trades or transfers.
