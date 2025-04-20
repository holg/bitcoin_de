// src/bin/backend.rs
#![cfg(feature = "backend")]
mod api_route;
mod handler;
use handler::{handle_show_account_info, handle_show_rates};
use {
    bitcoin_de::bitcoin_de_trading_api_sdk_v4::TradingApiSdkV4,
    // bitcoin_de::bitcoin_de_trading_api_sdk_v4::method_settings::constants::{
    //     METHOD_SHOW_ACCOUNT_INFO, METHOD_SHOW_RATES, SHOW_RATES_PARAMETER_TRADING_PAIR
    // },
    bitcoin_de::enums::TradingPair::*,
    // bitcoin_de::errors::ApiErrorDetail,
    // bitcoin_de::bitcoin_de_trading_api_sdk_v4::account::AccountInfoDetails,
    // bitcoin_de::bitcoin_de_trading_api_sdk_v4::ShowAccountInfoResponse,
    axum::{
        extract::FromRequest,
        response::IntoResponse,
        routing::get,
    },
    futures_util::stream::StreamExt,
    tokio::net::TcpListener,
    std::sync::Arc,
};
// This binary is only compiled when the "backend" feature is enabled in Cargo.toml.

#[tokio::main] // Use tokio runtime for the backend, enabled by the "backend" feature
async fn main() { // Async main function required for Axum and Tokio
    // Import Axum and related crates
    use axum::{routing::get, Router}; // Core Axum components
    use std::net::SocketAddr; // For defining the server address
    dotenv::dotenv().ok(); // Load environment variables from .env file
    // Define a simple handler function for the root route
    async fn hello_world() -> &'static str {
        "Hello from the Bitcoin.de Axum Backend!"
    }
    // Handler for the showAccountInfo endpoint
    // This handler calls the SDK's show_account_info method


    // --- Router Setup ---

    // Create an instance of the SDK client and wrap it in State to share it across handlers
    // In a real application, get credentials securely (e.g., environment variables)
    let api_key = std::env::var("API_KEY").expect("API_KEY not set for backend"); // Get credentials securely
    let api_secret = std::env::var("API_SECRET").expect("API_SECRET not set for backend"); // Get credentials securely
    let sdk_client = TradingApiSdkV4::new(api_key, api_secret);
    let sdk_client = Arc::new(sdk_client);
    // Wrap in State

    // Build our application router: define paths and their handlers
    let app = Router::new()
        .route("/", get(hello_world)) // Root route
        // Add the new route for account info. Map GET requests to the handle_show_account_info handler.
        .route("/api/v4/account/info", get(handle_show_account_info)) // Define the API endpoint route
        .route("/api/v4/rates/{trading_pair}", get(handle_show_account_info)) // Assuming this is the line // Define the API endpoint route
        // Add the SDK client State to the router's state so handlers can access it
        .with_state(sdk_client);

    // Define the address the server will bind to (e.g., localhost:3000)
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Axum backend listening on {}", addr);
    let listener = TcpListener::bind(&addr).await.unwrap();
    // Run the server
    // axum::Server::bind() creates a server instance
    // .serve() takes a service (our router) and starts serving requests
    // .await is needed because serve is an async operation
    // .expect() is a simple way to handle potential errors during server startup
    println!("Starting server at http://{}", addr);
    axum::serve(listener, app.into_make_service()).await.expect("Server error");
}

// Include a non-backend main function as a stub for other build targets.
// This prevents compilation errors when building cmdline or wasm targets,
// as the #[cfg(feature = "backend")] main function is not included in those builds.
#[cfg(not(feature = "backend"))]
fn main() {
    // This stub main function will be compiled when the "backend" feature is NOT enabled.
    // It can be empty or print a message indicating that the backend is not built.
    println!("Backend binary is not enabled in this build configuration.");
}
