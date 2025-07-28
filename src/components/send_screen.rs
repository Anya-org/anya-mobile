use dioxus::prelude::*;
use crate::adapters::WalletAdapter;
use crate::ports::WalletPort;
use anya_core::bitcoin::Address;
use std::str::FromStr;

#[component]
pub fn SendScreen(wallet: Signal<Box<WalletAdapter>>) -> Element {
    let mut recipient = use_signal(String::new);
    let mut amount = use_signal(String::new);
    let mut status_message = use_signal(String::new);
    let mut is_sending = use_signal(|| false);

    rsx! {
        div {
            style: "display: flex; flex-direction: column; height: 100%;",
            h2 { "Send" }
            form {
                style: "display: flex; flex-direction: column; gap: 16px;",
                onsubmit: move |_| {
                    if !*is_sending.read() {
                        is_sending.set(true);
                        status_message.set("".to_string()); // Clear previous status message
                        if let Ok(parsed_amount) = amount.read().parse::<f64>() {
                            if parsed_amount <= 0.0 {
                                status_message.set("Error: Amount must be positive".to_string());
                            } else if parsed_amount > (*wallet.read()).balance() {
                                status_message.set("Error: Insufficient balance".to_string());
                            } else {
                                match (*wallet.write()).send(recipient.read().clone(), parsed_amount) {
                                    Ok(_) => status_message.set("Transaction sent successfully!".to_string()),
                                    Err(e) => status_message.set(format!("Error sending transaction: {}", e)),
                                }
                            }
                        } else {
                            status_message.set("Error: Invalid amount".to_string());
                        }
                        is_sending.set(false);
                    }
                },
                input {
                    r#type: "text",
                    placeholder: "Recipient Address",
                    style: "padding: 12px; border-radius: 8px; border: 1px solid #ccc;",
                    value: "{recipient.read()}",
                    oninput: move |event| recipient.set(event.value().clone()),
                    disabled: *is_sending.read(),
                }
                input {
                    r#type: "text",
                    placeholder: "Amount",
                    style: "padding: 12px; border-radius: 8px; border: 1px solid #ccc;",
                    value: "{amount.read()}",
                    oninput: move |event| amount.set(event.value().clone()),
                    disabled: *is_sending.read(),
                }
                button {
                    style: "padding: 12px; border-radius: 8px; border: none; background: #4CAF50; color: white; cursor: pointer;",
                    disabled: *is_sending.read(),
                    "Send"
                }
            }
            if *is_sending.read() {
                p { "Sending transaction..." }
            } else {
                p {
                    style: "color: red;",
                    "{status_message}"
                }
            }
        }
    }
}
