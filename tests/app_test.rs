use anya_mobile::app;
use anya_mobile::ports::{WalletPort, Transaction};
use anya_mobile::adapters::WalletAdapter;
use dioxus::prelude::*;

#[test]
fn app_renders() {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_to_vec();
    let html = dioxus_ssr::render(&dom);
    assert!(html.contains("Anya Wallet"));
}

use anya_mobile::components::send_screen::SendScreen;
use anya_mobile::components::receive_screen::ReceiveScreen;

#[test]
fn send_screen_renders() {
    let wallet: Box<dyn WalletPort> = Box::new(WalletAdapter::new());
    let mut dom = VirtualDom::new_with_props(SendScreen, wallet);
    dom.rebuild_to_vec();
    let html = dioxus_ssr::render(&dom);
    assert!(html.contains("Recipient Address"));
    assert!(html.contains("Amount"));
    assert!(html.contains("Send"));
}

#[test]
fn receive_screen_renders() {
    let wallet: Box<dyn WalletPort> = Box::new(WalletAdapter::new());
    let mut dom = VirtualDom::new_with_props(ReceiveScreen, wallet);
    dom.rebuild_to_vec();
    let html = dioxus_ssr::render(&dom);
    assert!(html.contains("Your Wallet Address"));
    // Address may vary, so just check for the label
}
