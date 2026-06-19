use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct SignupResponse {
    pub user_id: Uuid,
    pub email: String,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
}
