use serde::{Deserialize, Serialize};

// For sqlx
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct UserModel {
    pub uid: String,
    pub name_last: String,
    pub name_first: String,
    pub email: String,
    pub phone: String,
    pub region: String,
    pub realm_id: String,
}

// For json response
#[derive(Debug, Deserialize, Serialize)]
pub struct UserModelResponse {
    pub uid: String,
    pub name_last: String,
    pub name_first: String,
    pub email: String,
    pub phone: String,
    pub region: String,
    pub realm_id: String,
}
