use std::sync::Arc;
use axum::{
    extract::{Path, Json, State},
    http::StatusCode,
    response::IntoResponse,
};
// mod csv_manager;

use crate::{
    api::user::user_schema::{CreateUserSchema, UpdateUserSchema}, csv::csv_manager::csv_manager::csv_read, AppState
};

use super::user_model::UserModelResponse;
use super::user_model::UserModel;

//AI

// User handlers...

pub async fn user_list_handler(State(data): State<Arc<AppState>>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Retrieve users from the database and convert to UserModelResponse
    // Implement the SQL query and retrieval logic here...

    // Placeholder for demonstration
    let users: Vec<UserModelResponse> = Vec::new();

    let mut temp = Vec::new();
   
    match csv_read("misc/MOCK_DATA.csv") {
        Ok(records) => {
            temp = records.clone();
            // for record in records {
            //     println!("{:?}", record);
            // }
        }
        Err(err) => {
         
            eprintln!("Error: {:?}", err);
        }
    }
    
    let json_response = serde_json::json!({
        "status": "okyyyyyy",
        "users": users,
        "temp": temp[1],
    });

    Ok(Json(json_response))
}

pub async fn create_user_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Insert user into the database
    // Implement the SQL insert logic here...

    // Placeholder for demonstration
    let uid = "dummy_uid".to_string();
    let user = UserModel {
        uid: uid.clone(),
        name_last: body.name_last.clone(),
        name_first: body.name_first.clone(),
        email: body.email.clone(),
        phone: body.phone.clone(),
        region: body.region.clone(),
        realm_id: body.realm_id.clone(),
    };

    let json_response = serde_json::json!({
        "status": "success",
        "data": serde_json::json!({
            "user": user,
        }),
    });

    Ok(Json(json_response))
}

pub async fn get_user_handler(
    Path(uid): Path<String>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Retrieve user from the database based on the provided UID
    // Implement the SQL query and retrieval logic here...

    // Placeholder for demonstration
    let user = UserModel {
        uid: uid.clone(),
        name_last: "dummy_last_name".to_string(),
        name_first: "dummy_first_name".to_string(),
        email: "dummy@example.com".to_string(),
        phone: "123456789".to_string(),
        region: "dummy_region".to_string(),
        realm_id: "dummy_realm_id".to_string(),
    };

    let json_response = serde_json::json!({
        "status": "success",
        "data": serde_json::json!({
            "user": user,
        }),
    });

    Ok(Json(json_response))
}

pub async fn edit_user_handler(
    Path(uid): Path<String>,
    State(_data): State<Arc<AppState>>,
    Json(body): Json<UpdateUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Update user in the database based on the provided UID
    // Implement the SQL update logic here...

    // Placeholder for demonstration
    let user = UserModel {
        uid: uid.clone(),
        name_last: body.name_last.clone().unwrap_or("dummy_last_name".to_string()),
        name_first: body.name_first.clone().unwrap_or("dummy_first_name".to_string()),
        email: body.email.clone().unwrap_or("dummy@example.com".to_string()),
        phone: body.phone.clone().unwrap_or("123456789".to_string()),
        region: body.region.clone().unwrap_or("dummy_region".to_string()),
        realm_id: body.realm_id.clone().unwrap_or("dummy_realm_id".to_string()),
    };

    let json_response = serde_json::json!({
        "status": "success",
        "data": serde_json::json!({
            "user": user,
        }),
    });

    Ok(Json(json_response))
}

pub async fn delete_user_handler(
    Path(uid): Path<String>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Delete user from the database based on the provided UID
    // Implement the SQL delete logic here...

    // Placeholder for demonstration

    let json_response = serde_json::json!({
        "status": "success",
        "message": format!("User with UID: {} deleted successfully", uid),
    });

    Ok(Json(json_response))
}