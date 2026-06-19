use axum::body::Body;
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;

use crate::middleware::role_guard::require_permission;

pub fn permission_guard(
    permission: &'static str,
) -> impl Fn(
    Request<Body>,
    Next,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Response> + Send>>
+ Clone {
    move |req, next| Box::pin(require_permission(permission, req, next))
}
