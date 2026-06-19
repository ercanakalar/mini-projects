use axum::{Router, middleware, routing::post};

use crate::{
    handlers::auth_handler::{
        create_user, forgot_password, login, logout, refresh_token, reset_password,
    },
    middleware::auth_middleware::auth_middleware,
    state::AppState,
};

pub fn auth_routes(state: AppState) -> Router<AppState> {
    let public = Router::new()
        .route("/signup", post(create_user))
        .route("/signin", post(login))
        .route("/forgot-password", post(forgot_password))
        .route("/reset-password/{token}", post(reset_password));

    let protected = Router::new()
        .route("/logout", post(logout))
        .route("/refresh-token", post(refresh_token))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    Router::new().merge(public).merge(protected)
}
