// bitcoin_de_trading_api_sdk_v4/enums.rs
#![allow(unused_doc_comments)]
/// A macro that generates an enum with string representation capabilities.
///
/// This macro creates an enum with the specified name and variants, and automatically
/// implements methods to convert enum variants to their string representations.
/// It also implements common traits like Debug, PartialEq, Eq, Hash, Clone, Copy,
/// and Display for the generated enum.
///
/// # Parameters
///
/// * `$name` - The identifier for the enum name.
/// * `$variant => $str_val` - Pairs of enum variant names and their corresponding string values.
///   Multiple pairs can be specified, separated by commas.
///
/// # Generated Methods
///
/// The macro generates the following methods for the enum:
/// * `as_str(&self) -> &'static str` - Returns the string representation of the enum variant.
/// * `to_string(&self) -> String` - Returns the string representation as an owned String.
///
/// # Example
///
/// ```
/// generate_enum!(Direction,
///     Up => "up",
///     Down => "down",
///     Left => "left",
///     Right => "right"
/// );
/// ```
macro_rules! generate_enum {
    ($name:ident, $($variant:ident => $str_val:expr),*) => {
        #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
        pub enum $name {
            $(
                $variant,
            )*
        }

        impl $name {
            // Generate as_str method for the enum
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(
                        $name::$variant => $str_val,
                    )*
                }
            }

            // Generate to_string method for the enum
            pub fn to_string(&self) -> String {
                self.as_str().to_string()
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.as_str())
            }
        }
    };
}

/// Represents the type of order in a trading operation.
///
/// This enum defines the possible order types that can be used when placing
/// trades on the Bitcoin.de platform. It distinguishes between buy and sell orders.
///
/// # Variants
///
/// * `Buy` - Represents a buy order, where the user wants to purchase an asset.
/// * `Sell` - Represents a sell order, where the user wants to sell an asset.
///
/// # Examples
///
/// ```
/// use bitcoin_de_trading_api_sdk_v4::enums::OrderType;
///
/// let order_type = OrderType::Buy;
/// assert_eq!(order_type.as_str(), "buy");
/// ```
generate_enum!(OrderType,
    Buy => "buy",
    Sell => "sell"
);

/// Represents the available trading pairs on the Bitcoin.de platform.
///
/// This enum defines all supported cryptocurrency trading pairs that can be used
/// for trading operations. Each variant represents a specific trading pair in the
/// format of base currency followed by quote currency (e.g., BTCEUR for Bitcoin/Euro).
///
/// # Variants
///
/// The enum includes various combinations of cryptocurrencies paired with:
/// - EUR (Euro)
/// - BTC (Bitcoin)
/// - ETH (Ethereum)
/// - BCH (Bitcoin Cash)
/// - BNB (Binance Coin)
/// - CHF (Swiss Franc)
///
/// # Examples
///
/// ```
/// use bitcoin_de_trading_api_sdk_v4::enums::TradingPair;
///
/// let pair = TradingPair::BTCEUR;
/// assert_eq!(pair.as_str(), "BTCEUR");
/// ```
generate_enum!(TradingPair,
    BTCEUR => "BTCEUR",
    BCHEUR => "BCHEUR",
    ETHBTC => "ETHBTC",
    ETHEUR => "ETHEUR",
    LTCEUR => "LTCEUR",
    LTCBTC => "LTCBTC",
    XRPEUR => "XRPEUR",
    XRPBTC => "XRPBTC",
    EOSEUR => "EOSEUR",
    EOSBTC => "EOSBTC",
    BNBEUR => "BNBEUR",
    BNBBTC => "BNBBTC",
    XMREUR => "XMREUR",
    XMRBTC => "XMRBTC",
    TRXEUR => "TRXEUR",
    TRXBTC => "TRXBTC",
    ETCBTC => "ETCBTC",
    ETCEUR => "ETCEUR",
    DASHEUR => "DASHEUR",
    DASHBTC => "DASHBTC",
    ZECEUR => "ZECEUR",
    ZECBTC => "ZECBTC",
    REPEUR => "REPEUR",
    REPBTC => "REPBTC",
    BATEUR => "BATEUR",
    BATBTC => "BATBTC",
    AIDUSDEUR => "AIDUSDEUR",
    AIDUSDBTC => "AIDUSDBTC",
    XLMEUR => "XLMEUR",
    XLMBTC => "XLMBTC",
    AVAXEUR => "AVAXEUR",
    AVAXBTC => "AVAXBTC",
    ADAEUR => "ADAEUR",
    ADABTC => "ADABTC",
    GRTEUR => "GRTEUR",
    GRTBTC => "GRTBTC",
    LINKEUR => "LINKEUR",
    LINKBTC => "LINKBTC",
    MATICBTC => "MATICBTC",
    MATICEUR => "MATICEUR",
    SOLEUR => "SOLEUR",
    SOLBTC => "SOLBTC",
    DOTEUR => "DOTEUR",
    DOTBTC => "DOTBTC",
    UNIEUR => "UNIEUR",
    UNIBTC => "UNIBTC",
    XMRETH => "XMRETH",
    XRPETH => "XRPETH",
    LTCETH => "LTCETH",
    DASHETH => "DASHETH",
    ZECETH => "ZECETH",
    REPBCH => "REPBCH",
    BATBCH => "BATBCH",
    XLMBCH => "XLMBCH",
    ADAETH => "ADAETH",
    GRTETH => "GRTETH",
    LINKETH => "LINKETH",
    MATICETH => "MATICETH",
    SOLETH => "SOLETH",
    DOTETH => "DOTETH",
    UNIBNB => "UNIBNB",
    EURCHF => "EURCHF",
    BTCCHF => "BTCCHF",
    ETHCHF => "ETHCHF"
);

/// Represents the available currencies on the Bitcoin.de platform.
///
/// This enum defines all supported cryptocurrencies and fiat currencies that can be used
/// in trading operations. Each variant represents a specific currency with its standard
/// ticker symbol.
///
/// # Variants
///
/// The enum includes various cryptocurrencies such as:
/// - BTC (Bitcoin)
/// - ETH (Ethereum)
/// - XRP (Ripple)
///
/// And fiat currencies:
/// - EUR (Euro)
/// - USD (US Dollar)
/// - CHF (Swiss Franc)
///
/// # Examples
///
/// ```
/// use bitcoin_de_trading_api_sdk_v4::enums::Currency;
///
/// let currency = Currency::BTC;
/// assert_eq!(currency.as_str(), "BTC");
/// ```
generate_enum!(Currency,
    BTC => "BTC",
    BCH => "BCH",
    ETH => "ETH",
    EUR => "EUR",
    LTC => "LTC",
    XRP => "XRP",
    EOS => "EOS",
    BNB => "BNB",
    XMR => "XMR",
    TRX => "TRX",
    ETC => "ETC",
    DASH => "DASH",
    ZEC => "ZEC",
    REP => "REP",
    BAT => "BAT",
    AIDUS => "AIDUS",
    XLM => "XLM",
    AVAX => "AVAX",
    ADA => "ADA",
    GRT => "GRT",
    LINK => "LINK",
    MATIC => "MATIC",
    SOL => "SOL",
    DOT => "DOT",
    UNI => "UNI",
    CHF => "CHF",
    USD => "USD"
);
const ALL_TRADING_PAIRS: &[TradingPair] = &[
    TradingPair::BTCEUR,
    TradingPair::BCHEUR,
    TradingPair::ETHBTC,
    TradingPair::ETHEUR,
    TradingPair::LTCEUR,
    TradingPair::LTCBTC,
    TradingPair::XRPEUR,
    TradingPair::XRPBTC,
    TradingPair::EOSEUR,
    TradingPair::EOSBTC,
    TradingPair::BNBEUR,
    TradingPair::BNBBTC,
    TradingPair::XMREUR,
    TradingPair::XMRBTC,
    TradingPair::TRXEUR,
    TradingPair::TRXBTC,
    TradingPair::ETCBTC,
    TradingPair::ETCEUR,
    TradingPair::DASHEUR,
    TradingPair::DASHBTC,
    TradingPair::ZECEUR,
    TradingPair::ZECBTC,
    TradingPair::REPEUR,
    TradingPair::REPBTC,
    TradingPair::BATEUR,
    TradingPair::BATBTC,
    TradingPair::AIDUSDEUR,
    TradingPair::AIDUSDBTC,
    TradingPair::XLMEUR,
    TradingPair::XLMBTC,
    TradingPair::AVAXEUR,
    TradingPair::AVAXBTC,
    TradingPair::ADAEUR,
    TradingPair::ADABTC,
    TradingPair::GRTEUR,
    TradingPair::GRTBTC,
    TradingPair::LINKEUR,
    TradingPair::LINKBTC,
    TradingPair::MATICBTC,
    TradingPair::MATICEUR,
    TradingPair::SOLEUR,
    TradingPair::SOLBTC,
    TradingPair::DOTEUR,
    TradingPair::DOTBTC,
    TradingPair::UNIEUR,
    TradingPair::UNIBTC,
    TradingPair::XMRETH,
    TradingPair::XRPETH,
    TradingPair::LTCETH,
    TradingPair::DASHETH,
    TradingPair::ZECETH,
    TradingPair::REPBCH,
    TradingPair::BATBCH,
    TradingPair::XLMBCH,
    TradingPair::ADAETH,
    TradingPair::GRTETH,
    TradingPair::LINKETH,
    TradingPair::MATICETH,
    TradingPair::SOLETH,
    TradingPair::DOTETH,
    TradingPair::UNIBNB,
    TradingPair::EURCHF,
    TradingPair::BTCCHF,
    TradingPair::ETHCHF,
];
impl TradingPair {
    /// Converts a string slice to a TradingPair enum variant using iteration.
    /// Returns an Error if the string does not match any variant.
    pub fn from_str(s: &str) -> Result<Self, crate::bitcoin_de_trading_api_sdk_v4::errors::Error> { // Return our custom Error
        let lower_s = s.to_ascii_lowercase(); // Convert input string to lowercase

        // Iterate over all variants, convert their string value to lowercase, and compare
        for pair in ALL_TRADING_PAIRS.iter() {
            if pair.as_str().to_ascii_lowercase() == lower_s {
                return Ok(*pair); // Return the found variant (TradingPair is Copy)
            }
        }

        // If no match is found
        Err(crate::bitcoin_de_trading_api_sdk_v4::errors::Error::Other(format!("Invalid trading pair: {}", s))) // Use a suitable error variant
    }
}