use dioxus::prelude::*;
use ui::storage::auth_state::AuthContext;

use crate::routes::AllRoute;

#[component]
pub fn ProtectedLayout() -> Element {
    let auth = use_context::<AuthContext>();

    let state = auth.read();

    if !state.initialized {
        return rsx! {
            div { "Loading..." }
        };
    }

    if !state.is_authenticated() {
        navigator().replace(AllRoute::SignInPage {});
        return rsx! {};
    }

    rsx! {
        Outlet::<AllRoute> {}
    }
}
