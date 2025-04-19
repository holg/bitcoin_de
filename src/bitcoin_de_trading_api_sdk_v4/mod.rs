// bitcoin_de_trading_api_sdk_v4/mod.rs
/// Bitcoin.de Trading API SDK v4 module
///
/// This module provides access to the Bitcoin.de Trading API v4, allowing
/// interaction with the Bitcoin.de cryptocurrency exchange platform.
pub mod constants;

/// Custom error types for the Bitcoin.de Trading API SDK.
///
/// Defines the various errors that can be returned by the SDK.
pub mod errors;

/// Enumeration types used throughout the Bitcoin.de Trading API
///
/// Contains various enum definitions that represent different states, types,
/// and options available in the Bitcoin.de Trading API.
pub mod enums;

/// Settings for API method calls
///
/// Provides configuration structures and options that can be used to customize
/// the behavior of individual API method calls.
pub mod method_settings;

/// Configuration for the Bitcoin.de Trading API client
///
/// Contains settings and options for configuring the API client, including
/// authentication details, request timeouts, and other global settings.
pub mod config;

/// Core implementation of the Trading API SDK
///
/// Implements the main functionality of the Trading API, including request
/// handling, authentication, and the various API endpoints.
pub mod trading_api_sdk_v4;

/// Re-export of the main Trading API SDK client
///
/// This is the primary entry point for interacting with the Bitcoin.de
/// Trading API v4.
pub use trading_api_sdk_v4::TradingApiSdkV4;

/// Re-export of the custom Error type
///
/// Makes the custom error type from the `errors` module directly available
/// at the crate root level, allowing users to access it without having to
/// import from the nested module.
pub use errors::Error;



pub mod responses;
pub use responses::*;