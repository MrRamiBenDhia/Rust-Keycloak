
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
pub struct RealmModelMessedUp {
    pub id: i64,
    pub name: Option<String>,
    pub description:  Option<String>,
    pub enabled: u8,
    pub verify_email: u8,
    pub remember_me: u8,
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
