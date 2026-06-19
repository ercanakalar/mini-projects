use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenResponse {
    pub user_id: Uuid,
    pub access_token: String,
    pub refresh_token: String,
}
