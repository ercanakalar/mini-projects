use dioxus::prelude::*;

pub type AuthContext = Signal<AuthState>;

#[derive(Clone, Debug)]
pub struct AuthState {
    pub token: Option<String>,
    pub initialized: bool,
}

impl Default for AuthState {
    fn default() -> Self {
        Self {
            token: None,
            initialized: false,
        }
    }
}

impl AuthState {
    pub fn is_authenticated(&self) -> bool {
        self.token.is_some()
    }
}
