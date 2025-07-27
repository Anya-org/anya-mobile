use dioxus::prelude::*;
use anya_core::bitcoin::wallet::Wallet;

#[component]
pub fn WalletScreen(wallet: Signal<Wallet>) -> Element {
    let wallet = wallet.read();
    let balance = wallet.balance();
    let transactions = wallet.transactions();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; height: 100%;",
            h2 { "Wallet" }
            div {
                style: "background: #f0f0f0; padding: 24px; border-radius: 12px; margin-bottom: 24px;",
                h3 { "Total Balance" }
                p { "{balance} BTC" }
            }
            div {
                style: "flex-grow: 1; border: 1px solid #eee; border-radius: 12px; padding: 24px;",
                h3 { "Transaction History" }
                ul {
                    style: "list-style: none; padding: 0;",
                    for transaction in transactions {
                        li {
                            style: "display: flex; justify-content: space-between; padding: 8px 0; border-bottom: 1px solid #eee;",
                            span { "{transaction.direction}" }
                            span { "{transaction.amount} BTC" }
                        }
                    }
                }
            }
        }
    }
}
