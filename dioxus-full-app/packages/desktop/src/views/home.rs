use dioxus::prelude::*;
use ui::{Hero, Login};

#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
        Login {}
    }
}
