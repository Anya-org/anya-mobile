use dioxus::prelude::*;
use crate::adapters::WalletAdapter;
use crate::ports::WalletPort;

#[component]
pub fn SendScreen(wallet: Signal<Box<WalletAdapter>>) -> Element {
    let mut recipient = use_signal(String::new);
    let mut amount = use_signal(String::new);

    rsx! {
        div {
            style: "display: flex; flex-direction: column; height: 100%;",
            h2 { "Send" }
            form {
                style: "display: flex; flex-direction: column; gap: 16px;",
                onsubmit: move |_| {
                    if let (Ok(parsed_amount), Ok(_)) = (amount.read().parse::<f64>(), recipient.read().parse::<String>()) {
                        (*wallet.write()).send(recipient.read().clone(), parsed_amount);
                    }
                },
                input {
                    r#type: "text",
                    placeholder: "Recipient Address",
                    style: "padding: 12px; border-radius: 8px; border: 1px solid #ccc;",
                    oninput: move |event| recipient.set(event.value().clone()),
                }
                input {
                    r#type: "text",
                    placeholder: "Amount",
                    style: "padding: 12px; border-radius: 8px; border: 1px solid #ccc;",
                    oninput: move |event| amount.set(event.value().clone()),
                }
                button {
                    style: "padding: 12px; border-radius: 8px; border: none; background: #4CAF50; color: white; cursor: pointer;",
                    "Send"
                }
            }
        }
    }
}
