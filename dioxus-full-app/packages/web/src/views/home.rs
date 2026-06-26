use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { id: "blog",

            // Content
            h1 { "HOME" }
        
        }
    }
}
