use dioxus::prelude::*;
use crate::adapters::WalletAdapter;
use crate::ports::WalletPort;

use std::sync::Arc;

#[component]
pub fn SendScreen(wallet: Arc<WalletAdapter>) -> Element {
    let mut recipient = use_signal(String::new);
    let mut amount_str = use_signal(String::new);
    let mut status_message = use_signal(String::new);

    let mut handle_send = move |_| {
        status_message.set("".to_string());
        let amount_btc = match amount_str.read().parse::<f64>() {
            Ok(a) => a,
            Err(_) => {
                status_message.set("Error: Invalid amount".to_string());
                return;
            }
        };

        if amount_btc <= 0.0 {
            status_message.set("Error: Amount must be positive".to_string());
            return;
        }

        let amount_sats = (amount_btc * 100_000_000.0) as u64;

        match wallet.balance() {
            Ok(balance_sats) => {
                if amount_sats > balance_sats {
                    status_message.set("Error: Insufficient balance".to_string());
                    return;
                }
            }
            Err(e) => {
                status_message.set(format!("Error fetching balance: {}", e));
                return;
            }
        }

        match wallet.send(recipient.read().clone(), amount_sats) {
            Ok(_) => status_message.set("Transaction sent successfully! (This is a mock)".to_string()),
            Err(e) => status_message.set(format!("Error sending transaction: {}", e)),
        }
    };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; height: 100%;",
            h2 { "Send" },
            form {
                style: "display: flex; flex-direction: column; gap: 16px;",
                onsubmit: move |e| {
                    e.stop_propagation();
                    handle_send(());
                },
                input {
                    r#type: "text",
                    placeholder: "Recipient Address",
                    style: "padding: 12px; border-radius: 8px; border: 1px solid #ccc;",
                    value: "{recipient.read()}",
                    oninput: move |event| recipient.set(event.value().clone()),
                },
                input {
                    r#type: "text",
                    placeholder: "Amount in BTC",
                    style: "padding: 12px; border-radius: 8px; border: 1px solid #ccc;",
                    value: "{amount_str.read()}",
                    oninput: move |event| amount_str.set(event.value().clone()),
                },
                button {
                    r#type: "submit",
                    style: "padding: 12px; border-radius: 8px; border: none; background: #4CAF50; color: white; cursor: pointer;",
                    "Send"
                }
            },
            p {
                // Display status message. Could be styled differently for success/error.
                "{status_message}"
            }
        }
    }
}
