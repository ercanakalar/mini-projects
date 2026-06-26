use dioxus::prelude::*;

mod guards;
mod layouts;
mod routes;
mod views;

use routes::routes::AllRoute;
use ui::storage::{auth_state::AuthState, storage::load_token};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut auth = use_context_provider(|| Signal::new(AuthState::default()));

    use_future(move || async move {
        let token = load_token().await;

        auth.set(AuthState {
            token,
            initialized: true,
        });
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Router::<AllRoute> {}
    }
}
