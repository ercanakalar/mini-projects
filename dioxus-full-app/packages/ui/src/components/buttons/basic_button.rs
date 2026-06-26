//! Reusable button component.
//!
//! Covers every button variant the login page needs: a primary submit
//! button (with optional loading spinner), a social-provider button (icon +
//! label), and a plain text link-button. Visual styling lives in
//! `button.css`; this file has no platform-specific code so it renders the
//! same on web, desktop, and mobile.

use dioxus::prelude::*;

/// Visual style of a [`Button`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    /// Solid, full-width call-to-action button (e.g. "Sign in").
    #[default]
    Primary,
    /// Outlined button used for third-party/social actions.
    Social,
    /// Bare text button styled like a link (e.g. "Forgot password?").
    Link,
}

impl ButtonVariant {
    fn class(&self) -> &'static str {
        match self {
            ButtonVariant::Primary => "btn btn-primary",
            ButtonVariant::Social => "btn btn-social",
            ButtonVariant::Link => "btn btn-link",
        }
    }
}

/// Props for [`Button`].
#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    /// Visual style. Defaults to [`ButtonVariant::Primary`].
    #[props(default)]
    pub variant: ButtonVariant,

    /// HTML button `type`. Defaults to `"button"` so buttons never
    /// accidentally submit a form unless explicitly set to `"submit"`.
    #[props(default = "button".to_string())]
    pub button_type: String,

    /// Disables the button and applies disabled styling.
    #[props(default = false)]
    pub disabled: bool,

    /// Shows a loading spinner in place of children and disables the
    /// button. Only meaningful for [`ButtonVariant::Primary`].
    #[props(default = false)]
    pub loading: bool,

    /// Text shown next to the spinner while `loading` is true.
    #[props(default = "Loading...".to_string())]
    pub loading_label: String,

    /// Click handler.
    pub onclick: EventHandler<()>,

    /// Button contents (label text, and/or an icon component).
    pub children: Element,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let is_disabled = props.disabled || props.loading;

    rsx! {
        button {
            r#type: "{props.button_type}",
            class: "{props.variant.class()}",
            disabled: is_disabled,
            onclick: move |_| {
                if !is_disabled {
                    props.onclick.call(());
                }
            },
            if props.loading {
                Spinner {}
                span { "{props.loading_label}" }
            } else {
                {props.children}
            }
        }
    }
}

#[component]
fn Spinner() -> Element {
    rsx! {
        svg {
            class: "spinner",
            view_box: "0 0 24 24",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            circle {
                style: "opacity: 0.25",
                cx: "12",
                cy: "12",
                r: "10",
                stroke: "currentColor",
                stroke_width: "4",
            }
            path {
                style: "opacity: 0.75",
                fill: "currentColor",
                d: "M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z",
            }
        }
    }
}
