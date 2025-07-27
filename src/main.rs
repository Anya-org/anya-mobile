
use dioxus::prelude::*;
use anya_core::api::handlers::wallet as wallet_handler;

fn main() {
    dioxus::launch(App);
}

fn App() -> Element {
    // State for wallet balance
    let mut balance = use_signal(|| None);

    // Fetch balance when component mounts
    use_future(move || async move {
        let fetched_balance = wallet_handler::get_balance().await;
        balance.set(Some(fetched_balance));
    });

    rsx! {
        div {
            style: "background: #f5f6fa; color: #222; font-family: sans-serif; padding: 32px;",
            h1 { "Anya Wallet" }
            div {
                style: "margin-top: 24px; background: #fff; border-radius: 12px; box-shadow: 0 2px 8px #eee; padding: 24px;",
                h2 { "Balance" }
                match *balance.read() {
                    Some(Ok(ref b)) => rsx!(p { "{b:?} ANYA" }),
                    Some(Err(_)) => rsx!(p { "Error fetching balance" }),
                    None => rsx!(p { "Loading..." }),
                }
            }
            // Add more UI components: Send, Receive, Transactions, etc.
        }
    }
}
