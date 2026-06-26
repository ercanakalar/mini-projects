//! Reusable text input component.
//!
//! Handles label, value binding, error display, and (for password fields)
//! a built-in show/hide toggle — so callers never need to manage that state
//! themselves. Styling lives in `input.css`.

use dioxus::prelude::*;

/// Which kind of input to render. Controls the HTML `type` attribute and
/// whether a show/hide toggle is rendered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputKind {
    #[default]
    Text,
    Email,
    Password,
}

impl InputKind {
    fn html_type(&self, reveal_password: bool) -> &'static str {
        match self {
            InputKind::Text => "text",
            InputKind::Email => "email",
            InputKind::Password => {
                if reveal_password {
                    "text"
                } else {
                    "password"
                }
            }
        }
    }

    fn autocomplete(&self) -> &'static str {
        match self {
            InputKind::Text => "off",
            InputKind::Email => "email",
            InputKind::Password => "current-password",
        }
    }
}

/// Props for [`TextInput`].
#[derive(Props, Clone, PartialEq)]
pub struct TextInputProps {
    /// Unique id, used to associate the `<label>` with the `<input>`.
    pub id: String,

    /// Label text shown above the field. Omit (or pass `None`) to render
    /// no label, e.g. when a sibling element already provides one.
    #[props(default)]
    pub label: Option<String>,

    /// Current value (controlled).
    pub value: String,

    /// Called with the new value on every keystroke.
    pub on_input: EventHandler<String>,

    /// Input kind — controls HTML type, autocomplete, and whether a
    /// password-visibility toggle is shown.
    #[props(default)]
    pub kind: InputKind,

    #[props(default)]
    pub placeholder: String,

    /// Validation/error message shown below the field, if any.
    #[props(default)]
    pub error: Option<String>,

    #[props(default = false)]
    pub disabled: bool,
}

#[component]
pub fn TextInput(props: TextInputProps) -> Element {
    // Password-visibility state is internal — callers never see or manage
    // it, matching the "self-managed" requirement.
    let mut reveal_password = use_signal(|| false);

    let html_type = props.kind.html_type(reveal_password());
    let input_class = if props.error.is_some() {
        "field-input has-error"
    } else {
        "field-input"
    };
    let is_password = matches!(props.kind, InputKind::Password);

    rsx! {
        div { class: "form-field",
            if let Some(label) = props.label.clone() {
                label { r#for: "{props.id}", class: "field-label", "{label}" }
            }

            div { class: "field-input-wrap",
                input {
                    id: "{props.id}",
                    r#type: "{html_type}",
                    autocomplete: "{props.kind.autocomplete()}",
                    placeholder: "{props.placeholder}",
                    class: if is_password { "{input_class} field-input-password" } else { "{input_class}" },
                    value: "{props.value}",
                    disabled: props.disabled,
                    oninput: move |evt| props.on_input.call(evt.value()),
                }

                if is_password {
                    button {
                        r#type: "button",
                        class: "password-toggle",
                        "aria-label": if reveal_password() { "Hide password" } else { "Show password" },
                        onclick: move |_| reveal_password.set(!reveal_password()),
                        EyeIcon { open: reveal_password() }
                    }
                }
            }

            if let Some(err) = props.error.clone() {
                span { class: "field-error", "{err}" }
            }
        }
    }
}

#[component]
fn EyeIcon(open: bool) -> Element {
    rsx! {
        if open {
            svg {
                width: "16",
                height: "16",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "1.8",
                xmlns: "http://www.w3.org/2000/svg",
                path {
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    d: "M3 3l18 18M10.584 10.587a2 2 0 002.828 2.83M9.363 5.365A9.466 9.466 0 0112 5c4.756 0 8.773 3.162 10.066 7.498a10.523 10.523 0 01-4.293 5.774M6.228 6.225C4.142 7.61 2.523 9.836 1.934 12.498A10.523 10.523 0 005.5 17.5",
                }
            }
        } else {
            svg {
                width: "16",
                height: "16",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "1.8",
                xmlns: "http://www.w3.org/2000/svg",
                path {
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    d: "M2.036 12.322a1.012 1.012 0 010-.639C3.423 7.51 7.36 4.5 12 4.5c4.638 0 8.573 3.007 9.963 7.178.07.207.07.431 0 .639C20.577 16.49 16.64 19.5 12 19.5c-4.638 0-8.573-3.007-9.963-7.178z",
                }
                path {
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    d: "M15 12a3 3 0 11-6 0 3 3 0 016 0z",
                }
            }
        }
    }
}
