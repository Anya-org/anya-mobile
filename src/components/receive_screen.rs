use dioxus::prelude::*;
use crate::adapters::WalletAdapter;
use crate::ports::WalletPort;

#[component]
pub fn ReceiveScreen(wallet: Signal<Box<WalletAdapter>>) -> Element {
    let wallet_address = wallet.read().address();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; height: 100%; align-items: center; justify-content: center; gap: 24px;",
            h2 { "Your Wallet Address" }
            div {
                style: "background: #f0f0f0; padding: 24px; border-radius: 12px; text-align: center;",
                p {
                    style: "font-family: monospace; font-size: 16px; margin-bottom: 16px;",
                    "{wallet_address}"
                }
                button {
                    style: "padding: 12px; border-radius: 8px; border: none; background: #007BFF; color: white; cursor: pointer;",
                    "Copy Address"
                }
            }
        }
    }
}
