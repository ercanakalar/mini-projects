use dioxus::prelude::*;

use crate::{
    guards::protected_guard::ProtectedLayout,
    layouts::Navbar,
    views::{Blog, Home, Profile, SignInPage},
};

#[derive(Clone, Routable, PartialEq)]
pub enum AllRoute {
    #[layout(Navbar)]
    #[route("/sign-in")]
    SignInPage {},

    #[layout(ProtectedLayout)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
    #[route("/profile")]
    Profile {},
}
