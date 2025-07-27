use anya_mobile::app;
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
    let mut dom = VirtualDom::new(SendScreen);
    dom.rebuild_to_vec();
    let html = dioxus_ssr::render(&dom);
    assert!(html.contains("Recipient Address"));
    assert!(html.contains("Amount"));
    assert!(html.contains("Send"));
}

#[test]
fn receive_screen_renders() {
    let mut dom = VirtualDom::new(ReceiveScreen);
    dom.rebuild_to_vec();
    let html = dioxus_ssr::render(&dom);
    assert!(html.contains("Your Wallet Address"));
    assert!(html.contains("bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh"));
}
