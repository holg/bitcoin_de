//! Command-line interface module for the Bitcoin.de Trading API client.
//!
//! This module defines the command-line arguments and related functionality
//! for the Bitcoin.de Trading API client application.

use clap::Parser;

/// Bitcoin-de Trading API SDK v4 Client
/// Command-line arguments for the Bitcoin.de Trading API client.
///
/// This struct defines the command-line arguments that can be passed to the application
/// for authenticating with the Bitcoin.de Trading API. Arguments can be provided via
/// command line or environment variables.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Your API Key from bitcoin.de
    ///
    /// This key is used for authentication with the Bitcoin.de API.
    /// Can be provided via command line or the API_KEY environment variable.
    #[arg(long = "api-key")]
    pub api_key: Option<String>,

    /// Your API Secret from bitcoin.de
    ///
    /// This secret is used for signing API requests to Bitcoin.de.
    /// Can be provided via command line or the API_SECRET environment variable.
    #[arg(long = "api-secret")]
    pub api_secret: Option<String>,

    /// Trading pairs to fetch rates for (comma-separated, e.g., btceur,etheur)
    ///
    /// Specify one or more trading pairs to get current rates.
    /// Example: --showrates btceur,etheur
    #[arg(long = "showrates")]
    pub showrates: Option<String>,

    /// Amounts to calculate values for (comma-separated, e.g., 1.5,3.5)
    ///
    /// Specify one or more amounts to multiply with the current rates.
    /// If multiple trading pairs are specified with --showrates, the amounts
    /// will be matched with trading pairs in the same order.
    /// Example: --amounts 1.5,3.5
    #[arg(long = "amounts")]
    pub amounts: Option<String>,

    /// CSV file to append results to
    ///
    /// If specified, the rate information will be appended to this CSV file.
    /// The file will be created if it doesn't exist.
    /// Example: --csv-output rates.csv
    #[arg(long = "csv-output")]
    pub csv_output: Option<String>,

    /// Generate SVG charts from CSV data
    #[clap(long)]
    pub generate_charts: Option<String>,

    /// Output directory for SVG charts (default: ./charts)
    #[clap(long, default_value = "charts")]
    pub charts_dir: String,

    /// Time range for chart generation in format "YYYY-MM-DD,YYYY-MM-DD"
    ///
    /// Optional parameter to filter chart data to a specific date range.
    /// If not provided, all available data in the CSV will be used.
    /// Example: --time-range 2023-01-01,2023-01-31
    #[arg(long)]
    pub time_range: Option<String>,
}

/// Parse command-line arguments and return the parsed arguments.
///
/// This function is a convenience wrapper around `Args::parse()`.
pub fn parse_args() -> Args {
    Args::parse()
}