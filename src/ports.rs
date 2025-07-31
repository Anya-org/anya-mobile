use crate::adapters::AdapterError;

pub trait WalletPort {
    fn address(&self) -> Result<String, AdapterError>;
    fn balance(&self) -> Result<u64, AdapterError>;
    fn transactions(&self) -> Result<Vec<Transaction>, AdapterError>;
    fn send(&self, recipient: String, amount: u64) -> Result<(), AdapterError>;
}

#[derive(Clone, Debug)]
pub struct Transaction {
    pub direction: String,
    pub amount: u64, // amount in satoshis
}
