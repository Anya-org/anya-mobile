use dioxus::prelude::*;
use crate::adapters::WalletAdapter;
use crate::ports::WalletPort;

#[component]
pub fn ReceiveScreen(wallet: Signal<Box<WalletAdapter>>) -> Element {
    let wallet_address = wallet.read().address();

    rsx! {
        div {
            "Receive Screen"
        }
    }
}
