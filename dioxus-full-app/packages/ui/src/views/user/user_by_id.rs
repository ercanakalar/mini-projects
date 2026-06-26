use dioxus::prelude::*;

use crate::models::user::user_data::GetUserByIdResponse;

const SIGNINCSS: Asset = asset!("/assets/styling/user_by_id.css");

#[component]
pub fn UserInfoCard(user: GetUserByIdResponse) -> Element {
    let display_name = if let Some(nick) = &user.nick_name {
        nick.clone()
    } else {
        format!(
            "{} {}",
            user.first_name.clone().unwrap_or_default(),
            user.last_name.clone().unwrap_or_default()
        )
        .trim()
        .to_string()
    };

    rsx! {
        document::Link { rel: "stylesheet", href: SIGNINCSS }

        div { class: "user-info",

            if let Some(photo) = &user.photo {
                img {
                    class: "user-avatar",
                    src: "{photo}",
                    alt: "{display_name}",
                }
            } else {
                div { class: "user-avatar placeholder",

                    "{display_name
                        .chars()
                        .next()
                        .unwrap_or('?')
                        .to_ascii_uppercase()}"
                }
            }

            div { class: "user-details",

                h3 { class: "user-name", "{display_name}" }

                if user.first_name.is_some() || user.last_name.is_some() {
                    p { class: "user-fullname",

                        "{user.first_name.clone().unwrap_or_default()} {user.last_name.clone().unwrap_or_default()}"
                    }
                }

                p { class: "user-id", "{user.id}" }
            }
        }
    }
}
