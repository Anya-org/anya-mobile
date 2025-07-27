
use dioxus::prelude::*;

mod components;
use components::{receive_screen::ReceiveScreen, send_screen::SendScreen, wallet_screen::WalletScreen};

fn main() {
    dioxus::launch(app);
}

#[derive(Clone, PartialEq)]
enum Tab {
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
            div {
                style: "display: flex; justify-content: space-around; background: #fff; border-radius: 12px; box-shadow: 0 2px 8px #eee; padding: 12px; margin-bottom: 24px;",
                button { onclick: move |_| current_tab.set(Tab::Wallet), "Wallet" }
                button { onclick: move |_| current_tab.set(Tab::Send), "Send" }
                button { onclick: move |_| current_tab.set(Tab::Receive), "Receive" }
            }
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
