use dioxus::prelude::*;

use crate::routes::AllRoute;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav {
            Link { to: AllRoute::Home {}, "Home" }

            Link { to: AllRoute::Blog { id: 1 }, "Blog" }

            Link { to: AllRoute::SignInPage {}, "Sign In" }

            Link { to: AllRoute::Profile {}, "Profile" }
        
        }

        Outlet::<AllRoute> {}
    }
}
