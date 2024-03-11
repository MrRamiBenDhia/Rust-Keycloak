use serde::{Deserialize, Serialize};

// Create
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateRealmSchema {
    pub name: String,
    pub description: String,
    pub enabled: Option<bool>,
    pub verify_email: Option<bool>,
    pub remember_me: Option<bool>,
}

// Update
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateRealmSchema {
    pub name: Option<String>,
    pub description: Option<String>,
    pub enabled: Option<bool>,
    pub verify_email: Option<bool>,
    pub remember_me: Option<bool>,
}