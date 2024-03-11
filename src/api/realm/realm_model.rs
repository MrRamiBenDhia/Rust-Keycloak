
use serde::{Deserialize, Serialize};

// For sqlx
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct RealmModel {
    pub id: String,
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub verify_email: bool,
    pub remember_me: bool,
}

// For json response
#[derive(Debug, Deserialize, Serialize)]
pub struct RealmModelResponse {
    pub id: String,
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub verify_email: bool,
    pub remember_me: bool,
}
