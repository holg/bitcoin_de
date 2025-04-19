// lib.rs
#![doc = include_str!("../README.md")]
/// Bitcoin.de Trading API SDK for API version 4
///
/// This module provides a Rust implementation of the Bitcoin.de Trading API v4,
/// allowing developers to interact with the Bitcoin.de cryptocurrency exchange
/// programmatically.
pub mod bitcoin_de_trading_api_sdk_v4;

/// Re-exports all items from the bitcoin_de_trading_api_sdk_v4 module
///
/// This allows users to import types and functions directly from the crate root
/// without having to specify the full module path.
pub use bitcoin_de_trading_api_sdk_v4::*;