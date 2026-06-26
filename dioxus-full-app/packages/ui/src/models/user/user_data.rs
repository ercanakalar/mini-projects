use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetUserByIdRequest {
    pub id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GetUserByIdResponse {
    pub id: Uuid,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub photo: Option<String>,
    pub nick_name: Option<String>,
    pub permit_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
