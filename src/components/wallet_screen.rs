use dioxus::prelude::*;
use crate::adapters::WalletAdapter;
use crate::ports::WalletPort;

use std::sync::Arc;

#[component]
pub fn WalletScreen(wallet: Arc<WalletAdapter>) -> Element {
    let balance_result = wallet.balance();
    let transactions_result = wallet.transactions();

    let balance_str = match balance_result {
        Ok(sats) => format!("{:.8} BTC", sats as f64 / 100_000_000.0),
        Err(e) => format!("Error: {}", e),
    };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; height: 100%;",
            h2 { "Wallet" }
            div {
                style: "background: #f0f0f0; padding: 24px; border-radius: 12px; margin-bottom: 24px;",
                h3 { "Total Balance" }
                p { "{balance_str}" }
            }
            div {
                style: "flex-grow: 1; border: 1px solid #eee; border-radius: 12px; padding: 24px;",
                h3 { "Transaction History" }
                match transactions_result {
                    Ok(txs) => {
                        if txs.is_empty() {
                            rsx! { li { "No transactions yet." } }
                        } else {
                            rsx! {
                                ul {
                                    style: "list-style: none; padding: 0;",
                                    for transaction in txs {
                                        li {
                                            style: "display: flex; justify-content: space-between; padding: 8px 0; border-bottom: 1px solid #eee;",
                                            span { "{transaction.direction}" }
                                            span { "{transaction.amount as f64 / 100_000_000.0} BTC" }
                                        }
                                    }
                                }
                            }
                        }
                    },
                    Err(e) => rsx! {
                        p { style: "color: red;", "Error fetching transactions: {e}" }
                    }
                }
            }
        }
    }
}
