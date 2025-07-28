use dioxus::prelude::*;
pub mod ports;
pub mod adapters;
use crate::adapters::WalletAdapter;

pub mod components;
use crate::components::{
    receive_screen::ReceiveScreen,
    send_screen::SendScreen,
    wallet_screen::WalletScreen,
    tab_bar::TabBar,
};

#[derive(Clone, PartialEq)]
pub enum Tab {
    Wallet,
    Send,
    Receive,
}

pub fn app() -> Element {
    let wallet: Signal<Box<WalletAdapter>> = use_signal(|| Box::new(WalletAdapter::new()));
    let current_tab = use_signal(|| Tab::Wallet);

    rsx! {
        div {
            style: "background: #f5f6fa; color: #222; font-family: sans-serif; padding: 32px; display: flex; flex-direction: column; height: 100vh;",
            h1 { "Anya Wallet" }
            TabBar { current_tab: current_tab.clone() }
            div {
                style: "flex-grow: 1; background: #fff; border-radius: 12px; box-shadow: 0 2px 8px #eee; padding: 24px;",
                match *current_tab.read() {
                    Tab::Wallet => rsx! { WalletScreen { wallet: wallet.clone() } },
                    Tab::Send => rsx! { SendScreen { wallet: wallet.clone() } },
                    Tab::Receive => rsx! { ReceiveScreen { wallet: wallet.clone() } },
                }
            }
        }
    }
}
