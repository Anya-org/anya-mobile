// Hexagonal architecture: Wallet adapter implementation
use crate::ports::{WalletPort, Transaction};
use anya_core::bitcoin::wallet::{Wallet as CoreWallet, WalletConfig, AddressType, BalanceManager, TransactionManager, WalletType, CoinSelectionStrategy, FeeStrategy, WalletError};
use anya_core::bitcoin::interface::BitcoinInterface;
use anya_core::bitcoin::Network;
use std::sync::Arc;
use std::path::PathBuf;

pub struct WalletAdapter {
    inner: CoreWallet,
}

impl WalletAdapter {
    pub fn new() -> Self {
        // Example config, replace with real config as needed
        let config = WalletConfig {
            wallet_type: WalletType::Standard,
            network: Network::Testnet,
            name: "anya-wallet".to_string(),
            seed_phrase: None,
            password: None,
            receive_descriptor: "".to_string(),
            change_descriptor: "".to_string(),
            xpub: None,
            data_dir: PathBuf::from("./"),
            use_rpc: false,
            coin_selection: CoinSelectionStrategy::LargestFirst,
            gap_limit: 20,
            min_confirmations: 1,
            fee_strategy: FeeStrategy::Medium,
        };
        // The bitcoin_client should implement BitcoinInterface, but for now use None
        let bitcoin_client: Option<Arc<dyn BitcoinInterface>> = None;
        Self { inner: CoreWallet::new(config, bitcoin_client) }
    }

    // Helper function to get the last error from anya_core (if available)
    fn last_error(&self) -> Option<WalletError> {
        // This is a placeholder. You would need to expose a way to get the last error from anya_core
        // For now, we'll assume anya_core returns errors directly from the methods.
        None
    }
}

impl WalletPort for WalletAdapter {
    fn address(&self) -> String {
        match self.inner.get_address(0, AddressType::Legacy) {
            Ok(addr) => addr.to_string(),
            Err(_) => "Error getting address".to_string(), // Improved error message
        }
    }

    fn balance(&self) -> f64 {
        BalanceManager::get_balance(&self.inner).unwrap_or(0) as f64
    }

    fn transactions(&self) -> Vec<Transaction> {
        TransactionManager::get_transactions(&self.inner, 20, 0)
            .unwrap_or_default()
            .into_iter()
            .map(|tx| {
                let amount = tx.amount as f64;
                let direction = if tx.is_incoming {
                    "in".to_string()
                } else {
                    "out".to_string()
                };
                Transaction {
                    direction,
                    amount,
                }
            })
            .collect()
    }

    fn send(&mut self, recipient: String, amount: f64) -> Result<(), String> {
        // Map anya_core errors to a String for now
        TransactionManager::create_and_send_transaction(&mut self.inner, vec![(recipient, amount as u64)])
            .map_err(|e| e.to_string())
    }
}
