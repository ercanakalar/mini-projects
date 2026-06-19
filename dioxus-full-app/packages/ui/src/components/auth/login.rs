use crate::services::auth::login;
use dioxus::prelude::*;

#[component]
pub fn Login() -> Element {
    let response = use_signal(|| String::new());
    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());

    rsx! {
        div {
            input {
                placeholder: "email",
                oninput: move |e| email.set(e.value()),
            }

            input {
                r#type: "password",
                placeholder: "password",
                oninput: move |e| password.set(e.value()),
            }

            button {
                onclick: move |_| {
                    let email = email();
                    let password = password();
                    let mut response = response;

                    async move {
                        match login("tes4t@test.com".to_string(), "123456".to_string()).await {
                            Ok(data) => response.set(data.access_token),
                            Err(err) => response.set(err.to_string()),
                        }
                    }
                },
                "Login"
            }

            p { "{response}" }
        }
    }
}
