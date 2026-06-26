use dioxus::prelude::*;
use ui::SignIn;

use crate::routes::AllRoute;

#[component]
pub fn SignInPage() -> Element {
    let nav = navigator();

    rsx! {
        SignIn {
            on_success: move |_| {
                nav.replace(AllRoute::Home {});
            },
        }
    }
}