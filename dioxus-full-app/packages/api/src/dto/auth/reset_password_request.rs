use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ResetPasswordRequest {
    pub password: String,
}
