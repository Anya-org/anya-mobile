use dioxus::prelude::*;
use crate::Tab;

#[component]
pub fn TabBar(current_tab: Signal<Tab>) -> Element {
    rsx! {
        div {
            style: "display: flex; justify-content: space-around; background: #fff; border-radius: 12px; box-shadow: 0 2px 8px #eee; padding: 12px; margin-bottom: 24px;",
            button { onclick: move |_| current_tab.set(Tab::Wallet), "Wallet" }
            button { onclick: move |_| current_tab.set(Tab::Send), "Send" }
            button { onclick: move |_| current_tab.set(Tab::Receive), "Receive" }
        }
    }
}
