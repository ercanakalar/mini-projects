use axum::routing::{get, put};
use axum::{Router, middleware};

use crate::middleware::permission_guard::permission_guard;
use crate::{
    handlers::user_handler::{
        get_all_users, get_user_by_id, update_permit_by_user_id, update_user,
    },
    middleware::auth_middleware::auth_middleware,
    state::AppState,
};

pub fn user_routes(state: AppState) -> Router<AppState> {
    let public = Router::new()
        .with_state(state.clone())
        .route("/{id}", get(get_user_by_id));

    let protected = Router::new()
        .with_state(state.clone())
        .route("/{id}", put(update_user))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    let admin_only = Router::new()
        .route("/all", get(get_all_users))
        .route("/permit/{id}", put(update_permit_by_user_id))
        .route_layer(middleware::from_fn(permission_guard("MANAGE_USERS")))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    Router::new()
        .merge(public)
        .merge(protected)
        .merge(admin_only)
}
