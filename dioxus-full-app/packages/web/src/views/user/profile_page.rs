use dioxus::{logger::tracing, prelude::*};
use uuid::Uuid;

use ui::{
    models::user::user_data::GetUserByIdResponse, services::user::get_user_by_id,
    views::user::user_by_id::UserInfoCard,
};

#[component]
pub fn Profile() -> Element {
    let mut user = use_signal(|| None::<GetUserByIdResponse>);

    use_effect(move || {
        spawn(async move {
            let id = match Uuid::parse_str("80af43ae-44ad-481e-b80c-cb0b7d29b205") {
                Ok(id) => id,
                Err(err) => {
                    tracing::error!("Invalid UUID: {err}");
                    return;
                }
            };
            match get_user_by_id(id).await {
                Ok(data) => user.set(Some(data)),
                Err(err) => tracing::error!("{err}"),
            }
        });
    });

    rsx! {
        if let Some(user) = user() {
            UserInfoCard { user }
        } else {
            p { "Loading..." }
        }
    }
}
