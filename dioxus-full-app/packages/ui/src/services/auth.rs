use crate::{
    models::{
        api_response::ApiResponse,
        auth::auth::{LoginRequest, LoginResponse, RefreshResponse},
    },
    services::api_client::client,
    storage::storage::load_refresh_token,
};

pub async fn login(email: String, password: String) -> Result<LoginResponse, String> {
    let response = client()
        .post("http://localhost:5000/api/auth/signin")
        .json(&LoginRequest { email, password })
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let api_response = response
        .json::<ApiResponse<LoginResponse>>()
        .await
        .map_err(|e| e.to_string())?;

    if !api_response.success {
        return Err(api_response.message);
    }

    api_response
        .data
        .ok_or_else(|| "No response data".to_string())
}

pub async fn refresh_token() -> Result<String, String> {
    let refresh_token = load_refresh_token().await.ok_or("No refresh token")?;

    let response = client()
        .post("http://localhost:5000/api/auth/refresh-token")
        .json(&serde_json::json!({
            "refresh_token": refresh_token
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let api_response = response
        .json::<ApiResponse<RefreshResponse>>()
        .await
        .map_err(|e| e.to_string())?;

    api_response
        .data
        .map(|d| d.access_token)
        .ok_or("No access token".to_string())
}
