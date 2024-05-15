use schemars::{JsonSchema, Map};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Params {
    #[serde(default)]
    pub version: u64,
    pub market_authorities: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MarketMap {
    pub markets: Map<String, Market>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Market {
    // Tickers is the full list of tickers and their associated configurations
    // to be stored on-chain.
    pub ticker: Ticker,
    pub provider_configs: Vec<ProviderConfig>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ProviderConfig {
    // Name corresponds to the name of the provider for which the configuration is
    // being set.
    pub name: String,
    // OffChainTicker is the off-chain representation of the ticker i.e. BTC/USD.
    // The off-chain ticker is unique to a given provider and is used to fetch the
    // price of the ticker from the provider.
    pub off_chain_ticker: String,
    // NormalizeByPair is the currency pair for this ticker to be normalized by.
    // For example, if the desired Ticker is BTC/USD, this market could be reached
    // using: OffChainTicker = BTC/USDT NormalizeByPair = USDT/USD This field is
    // optional and nullable.
    pub normalize_by_pair: CurrencyPair,
    // Invert is a boolean indicating if the BASE and QUOTE of the market should
    // be inverted. i.e. BASE -> QUOTE, QUOTE -> BASE
    #[serde(default)]
    pub invert: bool,
    // MetadataJSON is a string of JSON that encodes any extra configuration
    // for the given provider config.
    #[serde(rename(serialize = "metadata_JSON", deserialize = "metadata_JSON"))]
    pub metadata_json: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct CurrencyPair {
    #[serde(rename(serialize = "Base", deserialize = "Base"))]
    pub base: String,
    #[serde(rename(serialize = "Quote", deserialize = "Quote"))]
    pub quote: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Ticker {
    // CurrencyPair is the currency pair for this ticker.
    pub currency_pair: CurrencyPair,
    // Decimals is the number of decimal places for the ticker. The number of
    // decimal places is used to convert the price to a human-readable format.
    pub decimals: u64,
    // MinProviderCount is the minimum number of providers required to consider
    // the ticker valid.
    pub min_provider_count: u64,
    // Enabled is the flag that denotes if the Ticker is enabled for price
    // fetching by an oracle.
    #[serde(default)]
    pub enabled: bool,
    // MetadataJSON is a string of JSON that encodes any extra configuration
    // for the given ticker. ,
    #[serde(rename(serialize = "metadata_JSON", deserialize = "metadata_JSON"))]
    pub metadata_json: String,
}
