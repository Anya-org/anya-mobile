// Hexagonal architecture: Wallet port trait
use async_trait::async_trait;

#[async_trait]
pub trait WalletPort: Send + Sync {
    fn address(&self) -> String;
    fn balance(&self) -> f64;
    fn transactions(&self) -> Vec<Transaction>;
    fn send(&mut self, recipient: String, amount: f64) -> Result<(), String>;
}

#[derive(Clone, Debug)]
pub struct Transaction {
    pub direction: String,
    pub amount: f64,
}
