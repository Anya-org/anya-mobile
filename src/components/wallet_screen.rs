use dioxus::prelude::*;

#[component]
pub fn WalletScreen() -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; height: 100%;",
            h2 { "Wallet" }
            div {
                style: "background: #f0f0f0; padding: 24px; border-radius: 12px; margin-bottom: 24px;",
                h3 { "Total Balance" }
                p { "0.00000000 BTC" }
            }
            div {
                style: "flex-grow: 1; border: 1px solid #eee; border-radius: 12px; padding: 24px;",
                h3 { "Transaction History" }
                p { "No transactions yet." }
            }
        }
    }
}
