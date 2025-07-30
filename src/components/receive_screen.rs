use dioxus::prelude::*;
use crate::adapters::WalletAdapter;
use crate::ports::WalletPort;

#[component]
pub fn ReceiveScreen(wallet: Signal<Box<WalletAdapter>>) -> Element {
    let wallet_address = wallet.read().address();
    let wallet_address_clone = wallet_address.clone();

    let copy_address = move |_| {
        // TODO: Implement clipboard copy functionality
        #[cfg(all(not(target_arch = "wasm32"), not(target_os = "android")))] // Example: Clipboard access on non-web platforms
        if let Err(e) = arboard::Clipboard::new().and_then(|mut ctx| ctx.set_text(&wallet_address_clone)) {
            eprintln!("Failed to copy address to clipboard: {}", e);
        }

        #[cfg(target_arch = "wasm32")] // Example: Clipboard access on web platform
        if let Some(window) = web_sys::window() {
            if let Some(navigator) = window.navigator().unchecked_into::<web_sys::Navigator>().clipboard().as_ref() {
                 let _ = navigator.write_text(&wallet_address_clone);
            }
        }
    };

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
                    onclick: copy_address,
                    "Copy Address"
                }
            }
        }
    }
}
