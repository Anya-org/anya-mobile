// Hexagonal architecture: Wallet adapter implementation
use crate::ports::{WalletPort, Transaction};
use anya_core::bitcoin::wallet::{Wallet as CoreWallet, WalletConfig, AddressType, BalanceManager, TransactionManager, AddressManager, BitcoinInterface};
use std::sync::Arc;

pub struct WalletAdapter {
    inner: CoreWallet,
}

impl WalletAdapter {
    pub fn new() -> Self {
        // Example config, replace with real config as needed
        let config = WalletConfig::default();
        let bitcoin_client: Option<Arc<dyn std::any::Any + Send + Sync>> = None;
        Self { inner: CoreWallet::new(config, bitcoin_client) }
    }
}

impl WalletPort for WalletAdapter {
    fn address(&self) -> String {
        // Use index 0 and default address type (update as needed)
        self.inner.get_address(0, AddressType::default()).unwrap_or_default().to_string()
    }
    fn balance(&self) -> f64 {
        // Use BalanceManager trait
        BalanceManager::get_balance(&self.inner).unwrap_or(0.0)
    }
    fn transactions(&self) -> Vec<Transaction> {
        // Use TransactionManager trait
        TransactionManager::get_transactions(&self.inner, 20, 0)
            .unwrap_or_default()
            .into_iter()
            .map(|tx| Transaction {
                direction: tx.direction,
                amount: tx.amount,
            })
            .collect()
    }
    fn send(&mut self, recipient: String, amount: f64) -> Result<(), String> {
        // Implement send logic using CoreWallet API (placeholder)
        Err("Send not implemented".to_string())
    }
}
