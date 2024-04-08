use chrono::{DateTime, Local, NaiveDateTime};
use serde::{Deserialize, Serialize};

// For sqlx
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct UserModel {
    pub uid: String,
    pub user_role: String,
    pub name_first: String,
    pub name_last: String,
    pub email: String,
    pub phone: String,
    pub region: String,
    pub realm_id: String,
    pub created_at: DateTime<Local>,
}

// For json response
#[derive(Debug, Deserialize, Serialize)]
pub struct UserModelResponse {
    pub uid: String,
    pub user_role: String,
    pub name_last: String,
    pub name_first: String,
    pub email: String,
    pub phone: String,
    pub region: String,
    pub realm_id: String,
    // pub role : String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserModelResponseMessedUp {
    pub uid: i64,
    pub user_role:  Option<String>,
    pub name_last: Option<String>,
    pub name_first: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub region: Option<String>,
    // pub role : Option<String>,
    pub realm_id: Option<i64>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
