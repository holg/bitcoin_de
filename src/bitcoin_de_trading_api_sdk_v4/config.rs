// bitcoin_de_trading_api_sdk_v4/config.rs
/// Represents the API credentials required for authentication with the Bitcoin.de API.
///
/// This struct holds the API key and secret that are used to authenticate requests
/// to the Bitcoin.de trading platform.
///
/// TODO Refactor to use a more secure and efficient data structure for storing API credentials.
/// E.g. SecretBox
///
pub struct ApiCredentials {
    /// The API key provided by Bitcoin.de for authentication.
    /// This key identifies the user account making the API requests.
    pub api_key: String,

    /// The API secret provided by Bitcoin.de for authentication.
    /// This secret is used to sign API requests and should be kept secure.
    pub api_secret: String,
}