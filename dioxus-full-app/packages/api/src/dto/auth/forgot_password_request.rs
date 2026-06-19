use serde::Deserialize;

#[derive(Deserialize)]
pub struct ForgotPasswordRequest {
    pub email: String,
}
