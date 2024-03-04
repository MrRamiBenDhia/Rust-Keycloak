use serde::{Deserialize, Serialize};

// For sqlx
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct RealmModel {
    pub id: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

// For json response
#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct RealmModelResponse {
    pub id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}