// main.rs
#![allow(unused_imports)] // TODO well we shall remove this
use std::collections::HashMap;
use bitcoin_de::bitcoin_de_trading_api_sdk_v4::TradingApiSdkV4;
use bitcoin_de::bitcoin_de_trading_api_sdk_v4::method_settings::constants::{
    METHOD_SHOW_ACCOUNT_INFO, METHOD_SHOW_RATES, SHOW_RATES_PARAMETER_TRADING_PAIR};
use bitcoin_de::enums::TradingPair::*;
use std::env;
// We need tokio for the #[tokio::main] macro and the runtime for the default CLI
// This setup is conditional on the "default" feature being enabled.
#[cfg(feature = "cmdline")]
use {tokio, clap::Parser, dotenv::dotenv};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

/// Bitcoin-de Trading API SDK v4 Client
/// Command-line arguments for the Bitcoin.de Trading API client.
///
/// This struct defines the command-line arguments that can be passed to the application
/// for authenticating with the Bitcoin.de Trading API. Arguments can be provided via
/// command line or environment variables.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Your API Key from bitcoin.de
    ///
    /// This key is used for authentication with the Bitcoin.de API.
    /// Can be provided via command line or the API_KEY environment variable.
    #[arg(long = "api-key")]
    api_key: Option<String>,

    /// Your API Secret from bitcoin.de
    ///
    /// This secret is used for signing API requests to Bitcoin.de.
    /// Can be provided via command line or the API_SECRET environment variable.
    #[arg(long = "api-secret")]
    api_secret: Option<String>,
}

/// Entry point for the Bitcoin.de Trading API client application.
///
/// This function performs the following operations:
/// 1. Loads environment variables from a .env file (as alternative to command-line arguments)
/// 2. Parses command-line arguments
/// 3. Retrieves API credentials from arguments or environment variables
/// 4. Initializes the Trading API SDK
/// 5. Makes API calls to retrieve account information and current trading rates
///
/// The function demonstrates basic usage of the Bitcoin.de Trading API by:
/// - Calling the showAccountInfo endpoint to retrieve user account details
/// - Calling the showRates endpoint to get current trading rates for BTC/EUR
///
/// # Panics
///
/// This function will panic if:
/// - Neither command-line arguments nor environment variables provide valid API credentials
#[cfg(feature = "cmdline")]
#[tokio::main]
async fn  main() {
    // Load environment variables from .env file first.
    dotenv().ok();

    let args = Args::parse();

    let api_key = args.api_key
        .or_else(|| env::var("API_KEY").ok())
        .expect("API_KEY must be set either via --api-key argument or API_KEY environment variable");

    let api_secret = args.api_secret
        .or_else(|| env::var("API_SECRET").ok())
        .expect("API_SECRET must be set either via --api-secret argument or API_SECRET environment variable");

    let trading_api_sdk = TradingApiSdkV4::new(api_key, api_secret);

    // --- Call showAccountInfo ---
    println!("\n>>> Calling showAccountInfo...");
    let account_info_result = trading_api_sdk.show_account_info();
    match account_info_result.await {
        // Use {:?} for potentially complex Ok value, or handle specific structure later
        Ok(response) => println!("showAccountInfo successful Response: {:?}", response),
        Err(e) => eprintln!("Error during showAccountInfo: {}", e),
    }

    // --- Call showRates ---
    println!("\n>>> Calling showRates...");
    let show_rates_result = trading_api_sdk.show_rates(BTCEUR);

    match show_rates_result.await {
        // Use {:?} for potentially complex Ok value, or handle specific structure later
        Ok(response) => println!("showRates successful Response: {:?}", response),
        Err(e) => eprintln!("Error during showRates: {}", e),
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub async fn main() {


}