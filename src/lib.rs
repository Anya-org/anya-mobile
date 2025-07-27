use dioxus::prelude::*;

pub mod components;
use crate::components::{receive_screen::ReceiveScreen, send_screen::SendScreen, wallet_screen::WalletScreen, tab_bar::TabBar};

#[derive(Clone, PartialEq)]
pub enum Tab {
    Wallet,
    Send,
    Receive,
}

#[derive(Clone, PartialEq)]
pub struct Transaction {
    pub id: String,
    pub amount: f64,
    pub direction: String,
}

#[derive(Clone, PartialEq)]
pub struct WalletState {
    pub balance: f64,
    pub transactions: Vec<Transaction>,
}

pub fn app() -> Element {
    let mut current_tab = use_signal(|| Tab::Wallet);
    let wallet_state = use_signal(|| WalletState {
        balance: 0.5,
        transactions: vec![
            Transaction { id: "1".to_string(), amount: 0.1, direction: "Received".to_string() },
            Transaction { id: "2".to_string(), amount: 0.05, direction: "Sent".to_string() },
        ],
    });

    rsx! {
        div {
            style: "background: #f5f6fa; color: #222; font-family: sans-serif; padding: 32px; display: flex; flex-direction: column; height: 100vh;",
            h1 { "Anya Wallet" }
            TabBar { current_tab: current_tab }
            div {
                style: "flex-grow: 1; background: #fff; border-radius: 12px; box-shadow: 0 2px 8px #eee; padding: 24px;",
                match *current_tab.read() {
                    Tab::Wallet => rsx! { WalletScreen { wallet_state: wallet_state } },
                    Tab::Send => rsx! { SendScreen {} },
                    Tab::Receive => rsx! { ReceiveScreen {} },
                }
            }
        }
    }
}
