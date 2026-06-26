use dioxus::{hooks::use_context, signals::ReadableExt};
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client,
};

use crate::storage::auth_state::AuthContext;

pub fn client() -> Client {
    let mut headers = HeaderMap::new();

    let auth = use_context::<AuthContext>();

    let token = auth.read().token.clone();

    if let Some(token) = token {
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
        );
    }

    Client::builder().default_headers(headers).build().unwrap()
}
