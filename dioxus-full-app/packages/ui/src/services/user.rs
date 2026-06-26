use uuid::Uuid;

use crate::{
    models::{api_response::ApiResponse, user::user_data::GetUserByIdResponse},
    services::api_client::client,
};

pub async fn get_user_by_id(id: Uuid) -> Result<GetUserByIdResponse, String> {
    let response = client()
        .get(format!("http://localhost:5000/api/user/{id}"))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err(format!("HTTP {}", response.status()));
    }

    let api_response = response
        .json::<ApiResponse<GetUserByIdResponse>>()
        .await
        .map_err(|e| e.to_string())?;
    if !api_response.success {
        return Err(api_response.message);
    }

    api_response
        .data
        .ok_or_else(|| "No response data".to_string())
}
