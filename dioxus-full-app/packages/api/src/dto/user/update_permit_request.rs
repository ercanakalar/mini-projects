use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePermitRequest {
    pub user_id: Uuid,
    pub permit_id: Uuid,
}
