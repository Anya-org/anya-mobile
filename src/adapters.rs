use crate::ports::{Transaction, WalletPort};
use anya_core::bitcoin::wallet::{
    AddressManager, BalanceManager, TransactionManager, Wallet,
    WalletConfig as CoreWalletConfig, WalletError,
};
use anya_core::AnyaError;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use thiserror::Error;

// --- Custom Error Type ---
#[derive(Error, Debug)]
pub enum AdapterError {
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("Anya core wallet error: {0}")]
    CoreWallet(#[from] WalletError),
    #[error("Anya core error: {0}")]
    CoreAnya(#[from] AnyaError),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("TOML parsing error: {0}")]
    Toml(#[from] toml::de::Error),
    #[error("Mutex lock poisoned")]
    MutexPoison,
}

// --- Configuration Structs ---
#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub wallet: WalletConfigProxy,
    pub bitcoin_client: BitcoinClientConfig,
}

#[derive(Deserialize, Debug, Clone)]
pub struct WalletConfigProxy {
    pub name: String,
    pub network: String,
    pub seed_phrase: String,
}

#[derive(Deserialize, Debug)]
pub struct BitcoinClientConfig {
    pub client_type: String,
    pub url: String,
}

impl From<WalletConfigProxy> for CoreWalletConfig {
    fn from(proxy: WalletConfigProxy) -> Self {
        CoreWalletConfig {
            wallet_type: anya_core::bitcoin::wallet::WalletType::Standard,
            network: match proxy.network.as_str() {
                "Testnet" => anya_core::bitcoin::Network::Testnet,
                _ => anya_core::bitcoin::Network::Bitcoin,
            },
            name: proxy.name,
            seed_phrase: Some(proxy.seed_phrase),
            password: None,
            receive_descriptor: "".to_string(),
            change_descriptor: "".to_string(),
            xpub: None,
            data_dir: PathBuf::from("./wallet_data"),
            use_rpc: false,
            coin_selection: anya_core::bitcoin::wallet::CoinSelectionStrategy::LargestFirst,
            gap_limit: 20,
            min_confirmations: 1,
            fee_strategy: anya_core::bitcoin::wallet::FeeStrategy::Medium,
        }
    }
}

// --- Wallet Adapter Implementation ---
pub struct WalletAdapter {
    inner: Mutex<Wallet>,
    config: WalletConfigProxy,
}

// Manual implementation of PartialEq to satisfy Dioxus component requirements
impl PartialEq for WalletAdapter {
    fn eq(&self, other: &Self) -> bool {
        // We can't compare the inner wallet directly.
        // For the UI, we can consider two adapters equal if they have the same config.
        self.config.name == other.config.name
            && self.config.network == other.config.network
            && self.config.seed_phrase == other.config.seed_phrase
    }
}

impl WalletAdapter {
    pub fn new() -> Result<Self, AdapterError> {
        let app_config = Self::load_config()?;
        let bitcoin_client = Self::create_bitcoin_client(&app_config.bitcoin_client)?;
        let core_config: CoreWalletConfig = app_config.wallet.clone().into();

        let core_wallet_result: Result<anya_core::bitcoin::wallet::Wallet, WalletError> = Ok(anya_core::bitcoin::wallet::Wallet::new(core_config, Some(bitcoin_client)));
        match core_wallet_result {
            Ok(core_wallet) => Ok(Self {
                inner: Mutex::new(core_wallet),
                config: app_config.wallet,
            }),
            Err(e) => Err(AdapterError::CoreWallet(e)),
        }
    }

    fn load_config() -> Result<AppConfig, AdapterError> {
        let config_path = PathBuf::from("config.toml");
        if !config_path.exists() {
            return Err(AdapterError::Config(format!(
                "Config file not found at {:?}",
                config_path
            )));
        }
        let config_str = fs::read_to_string(config_path)?;
        let config: AppConfig = toml::from_str(&config_str)?;
        Ok(config)
    }

    fn create_bitcoin_client(
        _config: &BitcoinClientConfig,
    ) -> Result<Arc<dyn anya_core::bitcoin::interface::BitcoinInterface>, AdapterError> {
        todo!("Implement Bitcoin client creation (e.g., Electrum or RPC)");
    }
}

impl WalletPort for WalletAdapter {
    fn address(&self) -> Result<String, AdapterError> {
        let wallet = self.inner.lock().map_err(|_| AdapterError::MutexPoison)?;
        let address = wallet.get_new_address(anya_core::bitcoin::wallet::AddressType::Legacy)?;
        Ok(address.to_string())
    }

    fn balance(&self) -> Result<u64, AdapterError> {
        let wallet = self.inner.lock().map_err(|_| AdapterError::MutexPoison)?;
        let balance = wallet.get_balance()?;
        Ok(balance)
    }

    fn transactions(&self) -> Result<Vec<Transaction>, AdapterError> {
        let wallet = self.inner.lock().map_err(|_| AdapterError::MutexPoison)?;
        let tx_details = wallet.get_transactions(20, 0)?;

        let transactions = tx_details
            .into_iter()
            .map(|tx_info| Transaction {
                direction: tx_info.compute_txid().to_string(),
                amount: 0,
            })
            .collect();

        Ok(transactions)
    }

    fn send(&self, recipient: String, amount: u64) -> Result<(), AdapterError> {
        let wallet = self.inner.lock().map_err(|_| AdapterError::MutexPoison)?;
        let fee_rate = 0.0;
        let options = anya_core::bitcoin::wallet::transactions::TxOptions::default();

        let _psbt =
            wallet.create_transaction(vec![(recipient, amount)], fee_rate, options)?;

        todo!("Implement transaction signing and broadcasting");
    }
}
