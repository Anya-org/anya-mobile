
use dioxus::prelude::*;
use anya_core::api::handlers::wallet_handler;

fn main() {
    dioxus::desktop::launch(App);
}

fn App(cx: Scope) -> Element {
    // State for wallet balance
    let balance = use_state(cx, || None);

    // Fetch balance when component mounts
    use_effect(cx, (), |_| {
        // Replace with your actual async wallet function
        // Example: let fetched_balance = wallet_handler::get_balance();
        // For demonstration, we'll use a placeholder value
        let fetched_balance = 100.0; // TODO: Replace with wallet_handler::get_balance()
        balance.set(Some(fetched_balance));
        async {}
    });

    cx.render(rsx! {
        div {
            style: "background: #f5f6fa; color: #222; font-family: sans-serif; padding: 32px;",
            h1 { "Anya Wallet" }
            div {
                style: "margin-top: 24px; background: #fff; border-radius: 12px; box-shadow: 0 2px 8px #eee; padding: 24px;",
                h2 { "Balance" }
                match *balance.current() {
                    Some(ref b) => rsx!(p { "{b} ANYA" }),
                    None => rsx!(p { "Loading..." }),
                }
            }
            // Add more UI components: Send, Receive, Transactions, etc.
        }
    })
}
