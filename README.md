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

On bitcoin.de you need to set up this:
https://www.bitcoin.de/de/userprofile/tapi

Create a `.env` file in the project root with your Bitcoin.de API credentials:

```
API_KEY=your_bitcoin_de_api_key
API_SECRET=your_bitcoin_de_api_secret
```

You can use the provided `.env.sample` as a template.  
Alternatively, you can pass the `--api-key` and `--api-secret` to the command line.

## Usage

### As a Library

```rust
use bitcoin_de::bitcoin_de_trading_api_sdk_v4::trading_api_sdk_v4::TradingApiSdkV4;
use bitcoin_de::bitcoin_de_trading_api_sdk_v4::config::Config;

fn main() {
    // Create a new API client with your credentials
    let config = Config::new("your_api_key", "your_api_secret");
    let client = TradingApiSdkV4::new(config);
    
    // Get the current orderbook for BTC/EUR
    match client.show_orderbook("btceur", "buy") {
        Ok(response) => println!("Orderbook: {:?}", response),
        Err(e) => eprintln!("Error: {}", e),
    }
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
