use serde::{Deserialize, Serialize};

// For sqlx
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct UserModel {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

// For json response
#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct UserModelResponse {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}