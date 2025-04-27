#![cfg(feature = "cmdline")]
use chrono::NaiveDate;
use chrono::NaiveDateTime;
/// Bitcoin-de Trading API SDK v4 Client
use bitcoin_de::bitcoin_de_trading_api_sdk_v4::TradingApiSdkV4;
// use clap::Parser;
use std::env;
use dotenv::dotenv;
mod cli;
mod csv_util;
mod charts;

#[tokio::main]
async fn main() {
    // Load environment variables and parse arguments
    dotenv().ok();
    let args = cli::parse_args();

    // Initialize API client
    let api_client = match create_api_client(&args) {
        Ok(client) => client,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    // Handle commands based on provided arguments
    if let Some(_trading_pairs_str) = args.showrates.clone() {
        handle_show_rates_csv_command(&args).await;
        return;
    }

    if args.generate_charts.is_some() && args.csv_output.is_some() {
        // Pass time range parameters to chart generation if provided
        let time_range = args.time_range.as_deref();
        handle_generate_charts_command(
            &args.csv_output.unwrap(),
            &args.charts_dir,
            time_range
        );
        return;
    }

    // Execute default API calls if no specific command was requested
    execute_default_api_calls(&api_client).await;
}

/// Creates an API client using command-line arguments and environment variables
fn create_api_client(args: &cli::Args) -> Result<TradingApiSdkV4, String> {
    const API_KEY_ERROR: &str = "API_KEY must be set either via --api-key argument or API_KEY environment variable";
    const API_SECRET_ERROR: &str = "API_SECRET must be set either via --api-secret argument or API_SECRET environment variable";

    let api_key = args.api_key
        .clone()
        .or_else(|| env::var("API_KEY").ok())
        .ok_or(API_KEY_ERROR)?;

    let api_secret = args.api_secret
        .clone()
        .or_else(|| env::var("API_SECRET").ok())
        .ok_or(API_SECRET_ERROR)?;

    Ok(TradingApiSdkV4::new(api_key, api_secret))
}

/// Handles the show rates CSV command
async fn handle_show_rates_csv_command(args: &cli::Args) {
    // Split the comma-separated list of trading pairs
    let trading_pairs_str = args.showrates.as_ref().unwrap();
    let amounts_str = args.amounts.as_ref();
    let trading_pairs: Vec<&str> = trading_pairs_str.split(',').collect();
    let csv_file = &args.csv_output;
    let amounts: Vec<f64> = match amounts_str {
        Some(amounts) => amounts.split(',')
            .filter_map(|s| s.trim().parse::<f64>().ok())
            .collect(),
        None => vec![]
    };
    let api_client = create_api_client(&args);
    if let Err(err) = csv_util::get_rates_for_csv(&api_client.expect("API client creation failed"), trading_pairs, csv_file.as_deref(), amounts).await {
        eprintln!("Error generating CSV rates: {}", err);
    }
}

/// Handles chart generation from CSV files
///
/// # Arguments
/// * `csv_file` - Path to the CSV file containing rate data
/// * `charts_dir` - Directory where chart SVG files will be saved
/// * `time_range` - Optional time range specification (e.g., "2023-01-01,2023-01-31")
// --- In src/main.rs -> handle_generate_charts_command ---
fn handle_generate_charts_command(csv_file: &str, charts_dir: &str, time_range_str: Option<&str>) {
    let time_range_parsed: Option<(NaiveDateTime, NaiveDateTime)> = time_range_str.and_then(|range| {
        let parts: Vec<&str> = range.split(',').collect();
        if parts.len() == 2 {
            let start_res = NaiveDateTime::parse_from_str(parts[0].trim(), "%Y-%m-%d %H:%M:%S") // Adjust format if needed
                .or_else(|_| NaiveDate::parse_from_str(parts[0].trim(), "%Y-%m-%d").map(|d| d.and_hms_opt(0,0,0).unwrap())); // Try just date
            let end_res = NaiveDateTime::parse_from_str(parts[1].trim(), "%Y-%m-%d %H:%M:%S") // Adjust format if needed
                .or_else(|_| NaiveDate::parse_from_str(parts[1].trim(), "%Y-%m-%d").map(|d| d.and_hms_opt(23,59,59).unwrap())); // Try just date

            match (start_res, end_res) {
                (Ok(start), Ok(end)) => Some((start, end)),
                _ => {
                    eprintln!("Invalid time range format or date parsing failed. Expected 'YYYY-MM-DD[ HH:MM:SS],YYYY-MM-DD[ HH:MM:SS]'");
                    None
                }
            }
        } else {
            eprintln!("Invalid time range format. Expected 'start_date,end_date'");
            None
        }
    });

    if let Err(err) = charts::generate_charts_from_csv(
        csv_file,
        charts_dir,
        time_range_parsed, // Pass the parsed range
    ) {
        eprintln!("Error generating charts: {}", err);
    }
}


/// Executes default API calls to show account info and rates
///
/// This function demonstrates the usage of the TradingApiSdkV4 client
/// to call the `show_account_info` and `show_rates` methods.
/// It prints the results or errors to the console.
///
/// # Arguments
/// * `api_client` - A reference to the TradingApiSdkV4 client instance
///
/// # Example
/// ```
/// use bitcoin_de::bitcoin_de_trading_api_sdk_v4::TradingApiSdkV4;
/// // Assume `api_client` is already created and authenticated
/// execute_default_api_calls(&api_client).await;
/// ```
async fn execute_default_api_calls(api_client: &TradingApiSdkV4) {
    use bitcoin_de::enums::TradingPair::BTCEUR;

    // Call showAccountInfo
    println!("\n>>> Calling showAccountInfo...");
    match api_client.show_account_info().await {
        Ok(response) => println!("showAccountInfo successful Response: {:?}", response),
        Err(e) => eprintln!("Error during showAccountInfo: {}", e),
    }

    // Call showRates
    println!("\n>>> Calling showRates...");
    match api_client.show_rates(BTCEUR).await {
        Ok(response) => println!("showRates successful Response: {:?}", response),
        Err(e) => eprintln!("Error during showRates: {}", e),
    }
}

