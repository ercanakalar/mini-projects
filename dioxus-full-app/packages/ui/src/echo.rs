use dioxus::prelude::*;

const ECHO_CSS: Asset = asset!("/assets/styling/echo.css");

/// Echo component that demonstrates fullstack server functions.
#[component]
pub fn Echo() -> Element {
    let mut response = use_signal(|| String::new());

    rsx! {
        input {
            placeholder: "email",

            oninput: move |event| {
                let mut response = response;

                async move {
                    match login(event.value(), "password".to_string()).await {
                        Ok(data) => {
                            response.set(data.token);
                        }
                        Err(err) => {
                            response.set(format!("Error: {err}"));
                        }
                    }
                }
            }
        }

        p { "{response}" }
    }
}

async fn login(email: String, password: String) -> Result<LoginResponse, reqwest::Error> {
    println!("{}", email);
    reqwest::Client::new()
        .post("http://localhost:3000/api/auth/signin")
        .json(&LoginRequest { email, password })
        .send()
        .await?
        .json::<LoginResponse>()
        .await
}

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginResponse {
    pub token: String,
}
