
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserSchema {
    pub name_last: String,
    pub name_first: String,
    pub email: String,
    pub phone: String,
    pub region: String,
    pub realm_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateUserSchema {
    pub name_last: Option<String>,
    pub name_first: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub region: Option<String>,
    pub realm_id: Option<String>,
}