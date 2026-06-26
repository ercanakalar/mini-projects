use dioxus::prelude::*;

use crate::components::buttons::{Button, ButtonVariant};
use crate::components::inputs::{InputKind, TextInput};
use crate::services::auth::login;
use crate::storage::auth_state::{AuthContext, AuthState};
use crate::storage::storage::{save_refresh_token, save_token};

const SIGNINCSS: Asset = asset!("/assets/styling/sign_in.css");

#[component]
pub fn SignIn(on_success: EventHandler<()>) -> Element {
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut touched = use_signal(|| false);
    let mut is_loading = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);

    let mut auth = use_context::<AuthContext>();

    let email_error = use_memo(move || {
        let value = email.read();
        if !*touched.read() || value.is_empty() {
            None
        } else if !value.contains('@') || !value.contains('.') {
            Some("Enter a valid email address".to_string())
        } else {
            None
        }
    });

    let password_error = use_memo(move || {
        let value = password.read();
        if !*touched.read() || value.is_empty() {
            None
        } else if value.len() < 6 {
            Some("Password must be at least 6 characters".to_string())
        } else {
            None
        }
    });

    let mut submit_login = move || {
        touched.set(true);
        error.set(None);

        let email_val = email.read().clone();
        let password_val = password.read().clone();

        if email_val.is_empty() || password_val.is_empty() {
            return;
        }
        if !email_val.contains('@') || !email_val.contains('.') {
            return;
        }
        if password_val.len() < 6 {
            return;
        }

        is_loading.set(true);

        spawn(async move {
            match login(email_val, password_val).await {
                Ok(res) => {
                    save_token(&res.access_token).await;
                    save_refresh_token(&res.refresh_token).await;

                    auth.set(AuthState {
                        token: Some(res.access_token),
                        initialized: true,
                    });

                    on_success.call(());
                }
                Err(e) => error.set(Some(e.to_string())),
            }
            is_loading.set(false);
        });

        is_loading.set(false);
    };

    let handle_forgot_password = move || {};

    let handle_sign_up = move || {};

    // -------------------------------------------------------------------

    rsx! {
        document::Link { rel: "stylesheet", href: SIGNINCSS }
        div { class: "login-page",
            div { class: "login-container",

                div { class: "login-header",
                    div { class: "login-logo", "A" }
                    h1 { class: "login-title", "Welcome back" }
                    p { class: "login-subtitle", "Sign in to your account to continue" }
                }

                div { class: "login-card",

                    if let Some(err) = error() {
                        div { class: "login-error-banner",
                            span { "⚠" }
                            span { "{err}" }
                        }
                    }

                    TextInput {
                        id: "login-email",
                        label: Some("Email".to_string()),
                        kind: InputKind::Email,
                        placeholder: "you@example.com",
                        value: email(),
                        error: email_error(),
                        disabled: is_loading(),
                        on_input: move |val| email.set(val),
                    }

                    TextInput {
                        id: "login-password",
                        label: Some("Password".to_string()),
                        kind: InputKind::Password,
                        placeholder: "••••••••",
                        value: password(),
                        error: password_error(),
                        disabled: is_loading(),
                        on_input: move |val| password.set(val),
                    }

                    Button {
                        variant: ButtonVariant::Primary,
                        button_type: "button",
                        loading: is_loading(),
                        loading_label: "Signing in...",
                        // Submission is handled by the form's `onsubmit`
                        // above (native submit semantics, incl. Enter
                        // key) — this click handler intentionally does
                        // nothing extra to avoid double-firing.
                        onclick: move || { submit_login() },
                        "Sign in"
                    }
                    div { class: "field-label-row",
                        Button {
                            variant: ButtonVariant::Link,
                            onclick: move |_| handle_forgot_password(),
                            "Forgot password?"
                        }
                    }
                }

                p { class: "login-footer",
                    "Don't have an account? "
                    Button {
                        variant: ButtonVariant::Link,
                        onclick: move |_| handle_sign_up(),
                        "Sign up"
                    }
                }
            }
        }
    }
}
