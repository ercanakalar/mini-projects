use reqwest::Client;

use crate::models::{
    api_response::ApiResponse,
    auth::auth::{LoginRequest, LoginResponse},
};

pub async fn login(email: String, password: String) -> Result<LoginResponse, String> {
    let response = Client::new()
        .post("http://localhost:3000/api/auth/signin")
        .json(&LoginRequest { email, password })
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let body = response.text().await.map_err(|e| e.to_string())?;

    let api_response: ApiResponse<LoginResponse> =
        serde_json::from_str(&body).map_err(|e| format!("JSON parse error: {e}"))?;

    if !api_response.success {
        return Err(api_response.message);
    }

    api_response
        .data
        .ok_or_else(|| "Response data is empty".to_string())
}
