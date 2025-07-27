use dioxus::prelude::*;

#[component]
pub fn SendScreen() -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; height: 100%;",
            h2 { "Send" }
            form {
                style: "display: flex; flex-direction: column; gap: 16px;",
                input {
                    r#type: "text",
                    placeholder: "Recipient Address",
                    style: "padding: 12px; border-radius: 8px; border: 1px solid #ccc;",
                }
                input {
                    r#type: "text",
                    placeholder: "Amount",
                    style: "padding: 12px; border-radius: 8px; border: 1px solid #ccc;",
                }
                button {
                    style: "padding: 12px; border-radius: 8px; border: none; background: #4CAF50; color: white; cursor: pointer;",
                    "Send"
                }
            }
        }
    }
}
