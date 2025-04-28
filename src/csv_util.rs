#![cfg(feature = "cmdline")]
use std::{fs::OpenOptions, io::{Write, BufWriter}};
use chrono::Local;
use rust_decimal::prelude::ToPrimitive;
use bitcoin_de::{enums::TradingPair, TradingApiSdkV4};

/// Writes cryptocurrency exchange rate data to a CSV file.
///
/// This function appends the provided data to a CSV file at the specified path.
/// If the file doesn't exist, it will be created and a header row will be added.
/// If the file already exists, the data will be appended without adding a header.
///
/// # Parameters
///
/// * `file_path` - The path to the CSV file where data should be written.
/// * `data` - A slice of tuples containing the data to write, where each tuple contains:
///   * timestamp: String - The timestamp when the data was collected
///   * trading_pair: String - The trading pair identifier (e.g., "btceur")
///   * rate_weighted: f64 - The current weighted exchange rate
///   * rate_weighted_3h: f64 - The 3-hour weighted exchange rate
///   * rate_weighted_12h: f64 - The 12-hour weighted exchange rate
///   * amount: f64 - The amount of the base currency
///   * value_weighted: f64 - The value in quote currency using the current weighted rate
///   * value_weighted_3h: f64 - The value in quote currency using the 3-hour weighted rate
///   * value_weighted_12h: f64 - The value in quote currency using the 12-hour weighted rate
///
/// # Returns
///
/// * `std::io::Result<()>` - Ok(()) if the write operation was successful, or an error if it failed.
///
/// # Examples
///
/// ```
/// let data = vec![
///     ("2023-01-01 12:00:00".to_string(), "btceur".to_string(),
///      50000.0, 49500.0, 49000.0, 0.5, 25000.0, 24750.0, 24500.0)
/// ];
/// write_to_csv("rates.csv", &data)?;
/// ```
pub fn write_to_csv(
    file_path: &str,
    data: &[(String, String, f64, f64, f64, f64, f64, f64, f64)]
) -> std::io::Result<()> {
    // Check if the file exists
    let file_exists = std::path::Path::new(file_path).exists();

    // Open the file in append mode, create if it doesn't exist
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)  // Append instead of truncate
        .open(file_path)?;

    let mut writer = BufWriter::new(file);

    // Write header only if the file is new (doesn't exist)
    if !file_exists {
        writeln!(writer, "timestamp,trading_pair,rate_weighted,rate_weighted_3h,rate_weighted_12h,amount,value_weighted,value_weighted_3h,value_weighted_12h")?;
    }

    // Write all data rows
    for (timestamp, trading_pair, rate_weighted, rate_weighted_3h, rate_weighted_12h,
        amount, value_weighted, value_weighted_3h, value_weighted_12h) in data {
        writeln!(
            writer,
            "{},{},{},{},{},{},{},{},{}",
            timestamp,
            trading_pair,
            rate_weighted,
            rate_weighted_3h,
            rate_weighted_12h,
            amount,
            value_weighted,
            value_weighted_3h,
            value_weighted_12h
        )?;
    }

    Ok(())
}

/// Retrieves cryptocurrency exchange rates for specified trading pairs and optionally writes the data to a CSV file.
///
/// This function fetches current weighted rates (instant, 3-hour, and 12-hour) for each provided trading pair
/// using the Bitcoin.de API. If amounts are provided, it also calculates the corresponding values in the quote
/// currency. Results are displayed in the console and optionally written to a CSV file.
///
/// # Parameters
///
/// * `trading_api_sdk` - A reference to the initialized Bitcoin.de Trading API SDK client.
/// * `trading_pairs` - A vector of strings representing the trading pairs to query (e.g., "btceur", "etheur").
/// * `csv_output` - An optional file path where the data should be written as CSV. If `None`, no CSV is created.
/// * `amounts` - A vector of amounts in the base currency for each trading pair. If an amount is provided and
///   greater than 0.0, calculations will be performed and API calls will be made. Otherwise, dummy values of 0
///   will be used.
///
/// # Returns
///
/// * `std::io::Result<()>` - Ok(()) if the operation was successful, or an error if CSV writing failed.
///
/// # Examples
///
/// ```
/// let sdk = TradingApiSdkV4::new("api_key", "api_secret");
/// let trading_pairs = vec!["btceur", "etheur"];
/// let amounts = vec![0.5, 1.0];
/// get_rates_for_csv(&sdk, trading_pairs, Some("rates.csv"), amounts).await?;
/// ```
pub async fn get_rates_for_csv(trading_api_sdk: &TradingApiSdkV4, trading_pairs:Vec<&str>, csv_output: Option<&str>, amounts:Vec<f64>) -> std::io::Result<()> {
    // Create a vector to collect CSV data
    let mut csv_data = Vec::new();

    // Process each trading pair
    for (i, trading_pair_str) in trading_pairs.iter().enumerate() {
        let trading_pair_str = trading_pair_str.trim();

        // Determine if we have a valid amount (exists and is greater than 0.0)
        let amount_opt = if i < amounts.len() && amounts[i] > 0.0 {
            Some(amounts[i])
        } else {
            None
        };

        println!("\n>>> Calling showRates for {}...", trading_pair_str);

        // Convert string to TradingPair enum using the from_str method
        let trading_pair = match TradingPair::from_str(trading_pair_str) {
            Ok(pair) => pair,
            Err(e) => {
                eprintln!("Error: {}", e);
                eprintln!("Please provide a valid trading pair (e.g., btceur, etheur)");
                continue; // Skip this pair and continue with the next one
            }
        };

        // Call the API to get rates only if we have a valid amount
        let show_rates_result = if amount_opt.is_some() {
            // We have a valid amount, so call the API
            trading_api_sdk.show_rates(trading_pair).await
        } else {
            // No valid amount, create a dummy result with zeros
            Ok(bitcoin_de::bitcoin_de_trading_api_sdk_v4::responses::misc::ShowRatesResponse {
                trading_pair: trading_pair.as_str().to_string(),
                rates: bitcoin_de::bitcoin_de_trading_api_sdk_v4::responses::misc::RatesDetails {
                    rate_weighted: rust_decimal::Decimal::new(0, 0),
                    rate_weighted_3h: rust_decimal::Decimal::new(0, 0),
                    rate_weighted_12h: rust_decimal::Decimal::new(0, 0),
                },
                errors: Vec::new(),
                credits: 0,
            })
        };

        match show_rates_result {
            Ok(response) => {
                // Only print rates if we actually called the API
                if amount_opt.is_some() {
                    println!("Current rates for {}:", trading_pair_str.to_uppercase());
                    println!("  Weighted rate: {}", response.rates.rate_weighted);
                    println!("  3h weighted rate: {}", response.rates.rate_weighted_3h);
                    println!("  12h weighted rate: {}", response.rates.rate_weighted_12h);
                }

                // Get the base currency (first 3 characters of the trading pair)
                let base_currency = trading_pair.as_str().chars().take(3).collect::<String>();
                let quote_currency = trading_pair.as_str().chars().skip(3).take(3).collect::<String>();

                // Convert Decimal to f64 for calculations
                let rate_weighted = response.rates.rate_weighted.to_f64().unwrap_or(0.0);
                let rate_weighted_3h = response.rates.rate_weighted_3h.to_f64().unwrap_or(0.0);
                let rate_weighted_12h = response.rates.rate_weighted_12h.to_f64().unwrap_or(0.0);

                // Only show calculations if we have a valid amount
                if let Some(amount) = amount_opt {
                    println!("\nCalculations for {} {}:", amount, base_currency);
                    println!("  Value (weighted rate): {} {}",
                             amount * rate_weighted,
                             quote_currency);
                    println!("  Value (3h weighted): {} {}",
                             amount * rate_weighted_3h,
                             quote_currency);
                    println!("  Value (12h weighted): {} {}",
                             amount * rate_weighted_12h,
                             quote_currency);
                }

                // Collect data for CSV
                if let Some(_csv_file) = csv_output {
                    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

                    // Calculate values if amount is provided
                    let (amount_val, value_weighted, value_weighted_3h, value_weighted_12h) =
                        if let Some(amt) = amount_opt {
                            (
                                amt,
                                amt * rate_weighted,
                                amt * rate_weighted_3h,
                                amt * rate_weighted_12h
                            )
                        } else {
                            (0.0, 0.0, 0.0, 0.0)
                        };

                    // Add data to collection
                    csv_data.push((
                        timestamp,
                        trading_pair_str.to_string(),
                        rate_weighted,
                        rate_weighted_3h,
                        rate_weighted_12h,
                        amount_val,
                        value_weighted,
                        value_weighted_3h,
                        value_weighted_12h
                    ));
                }
            },
            Err(e) => eprintln!("Error during showRates for {}: {}", trading_pair_str, e),
        }
    }

    // Write all collected data to CSV file at once
    if let Some(csv_file) = &csv_output {
        if !csv_data.is_empty() {
            match write_to_csv(csv_file, &csv_data) {
                Ok(_) => println!("\nAll data written to CSV file: {}", csv_file),
                Err(e) => eprintln!("\nError writing to CSV file: {}", e),
            }
        }
    }
    Ok(())
}