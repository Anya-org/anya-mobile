use dioxus::prelude::*;
use crate::adapters::WalletAdapter;
use crate::ports::WalletPort;

use std::sync::Arc;

#[component]
pub fn ReceiveScreen(wallet: Arc<WalletAdapter>) -> Element {
    let wallet_address = wallet.address();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; height: 100%; align-items: center; justify-content: center; gap: 24px;",
            h2 { "Your Wallet Address" }
            div {
                style: "background: #f0f0f0; padding: 24px; border-radius: 12px; text-align: center;",
                match wallet_address {
                    Ok(address) => {
                        let addr_clone = address.clone();
                        let copy_address = move |_| {
                             if let Err(e) = arboard::Clipboard::new().and_then(|mut ctx| ctx.set_text(&addr_clone)) {
                                eprintln!("Failed to copy address to clipboard: {}", e);
                            }
                        };
                        rsx! {
                            p {
                                style: "font-family: monospace; font-size: 16px; margin-bottom: 16px; word-break: break-all;",
                                "{address}"
                            }
                            button {
                                style: "padding: 12px; border-radius: 8px; border: none; background: #007BFF; color: white; cursor: pointer;",
                                onclick: copy_address,
                                "Copy Address"
                            }
                        }
                    },
                    Err(e) => rsx! {
                        p {
                            style: "color: red; font-family: monospace; font-size: 16px; margin-bottom: 16px;",
                            "Error: {e}"
                        }
                    }
                }
            }
        }
    }
}
