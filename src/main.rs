use dioxus::prelude::*;

mod components;
use components::{receive_screen::ReceiveScreen, send_screen::SendScreen, wallet_screen::WalletScreen, tab_bar::TabBar};

fn main() {
    dioxus::launch(app);
}

#[derive(Clone, PartialEq)]
pub enum Tab {
    Wallet,
    Send,
    Receive,
}

fn app() -> Element {
    let mut current_tab = use_signal(|| Tab::Wallet);

    rsx! {
        div {
            style: "background: #f5f6fa; color: #222; font-family: sans-serif; padding: 32px; display: flex; flex-direction: column; height: 100vh;",
            h1 { "Anya Wallet" }
            TabBar { current_tab: current_tab }
            div {
                style: "flex-grow: 1; background: #fff; border-radius: 12px; box-shadow: 0 2px 8px #eee; padding: 24px;",
                match *current_tab.read() {
                    Tab::Wallet => rsx! { WalletScreen {} },
                    Tab::Send => rsx! { SendScreen {} },
                    Tab::Receive => rsx! { ReceiveScreen {} },
                }
            }
        }
    }
}
