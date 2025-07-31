use anya_mobile::app;
use dioxus::prelude::*;
use anya_mobile::adapters::WalletAdapter;

#[test]
fn app_renders() {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_to_vec();
    let html = dioxus_ssr::render(&dom);
    assert!(html.contains("Anya Wallet"));
}

use anya_mobile::components::send_screen::SendScreen;
use anya_mobile::components::receive_screen::ReceiveScreen;

use std::sync::Arc;

#[test]
fn send_screen_renders() {
    let mut dom = VirtualDom::new(|| {
        let wallet = Arc::new(WalletAdapter::new().unwrap());
        rsx! { SendScreen { wallet: wallet } }
    });
    dom.rebuild_to_vec();
    let html = dioxus_ssr::render(&dom);
    assert!(html.contains("Recipient Address"));
    assert!(html.contains("Amount in BTC"));
    assert!(html.contains("Send"));
}

#[test]
fn receive_screen_renders() {
    let mut dom = VirtualDom::new(|| {
        let wallet = Arc::new(WalletAdapter::new().unwrap());
        rsx! { ReceiveScreen { wallet: wallet } }
    });
    dom.rebuild_to_vec();
    let html = dioxus_ssr::render(&dom);
    assert!(html.contains("Your Wallet Address"));
}
