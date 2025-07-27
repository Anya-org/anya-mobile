use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tab {
    Wallet,
    Send,
    Receive,
}

fn app() -> Element {
    let current_tab = use_signal(|| Tab::Wallet);

    rsx! {
        div {
            style: "background: #f5f6fa; color: #222; font-family: sans-serif; padding: 32px;",
            h1 { "Anya Wallet" }
            TabBar { current_tab: current_tab.clone() }
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

#[component]
fn TabBar(current_tab: Signal<Tab>) -> Element {
    let tab_button = |tab: Tab, label: &str| {
        let is_active = *current_tab.read() == tab;
        let style = if is_active {
            "font-weight: bold; color: #1976d2;"
        } else {
            ""
        };
        rsx! {
            button {
                style: "{style}",
                onclick: move |_| current_tab.set(tab),
                "{label}"
            }
        }
    };

    rsx! {
        div {
            style: "display: flex; justify-content: space-around; background: #fff; border-radius: 12px; box-shadow: 0 2px 8px #eee; padding: 12px; margin-bottom: 24px;",
            {tab_button(Tab::Wallet, "Wallet")}
            {tab_button(Tab::Send, "Send")}
            {tab_button(Tab::Receive, "Receive")}
        }
    }
}

// Placeholder components for demonstration
#[component]
fn WalletScreen() -> Element {
    rsx! { div { "Wallet Screen" } }
}

#[component]
fn SendScreen() -> Element {
    rsx! { div { "Send Screen" } }
}

#[component]
fn ReceiveScreen() -> Element {
    rsx! { div { "Receive Screen" } }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus::prelude::*;

    #[tokio::test]
    async fn test_wallet_handler_get_balance_async() {
        assert!(true, "Replace with real async wallet_handler test");
    }

    // Example of a Dioxus component test (requires dioxus-testing crate)
    // #[test]
    // fn test_app_component_renders() {
    //     use dioxus_testing::prelude::*;
    //     let mut tester = VirtualDom::new(app);
    //     tester.rebuild();
    //     let html = tester.render_to_string();
    //     assert!(html.contains("Anya Wallet"));
    // }
}
