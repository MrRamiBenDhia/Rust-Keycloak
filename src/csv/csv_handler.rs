use axum::{
    extract::{Json, Path, Query, State}, http::{Response, StatusCode}, response::IntoResponse, routing::post, Router
};
use chrono::{Local, NaiveDateTime};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::csv_manager::{
    csv_deserialize, csv_read, csv_write, delete_csv,
};
use crate::{
    api::user::user_model::{UserModel, UserModelResponse, UserModelResponseMessedUp}, csv::csv_manager::csv_my_custom_deserialize, AppState
};
pub async fn create_new_users_from_csv(
    // Path(filename): Path<String>,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    //! bismilah !
    
    // Read CSV file
    let csv_data = match csv_my_custom_deserialize("misc/Rust_User_3.csv") {
        Ok(data) => data,
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": err.to_string() }))))
    };


    // Call function to insert users into database
    match post_users_request(&csv_data, &app_state).await {
        Ok(_) => {
            let response = serde_json::json!({
                "status": "success",
                "message": "Users created successfully from CSV",
                "result": csv_data
            });
            Ok((StatusCode::OK, Json(response)))
        }
        Err(err) => {
            let error_message = format!("Error returned from database: {:?}", err);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": error_message }))));
        }
    }
}

 pub async fn user_list_to_csv(
    // Path(filename): Path<String>,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Fetch user data from the database (add your logic here)
    
    let timestamp: chrono::prelude::DateTime<Local> = Local::now();


    let users = vec![
        UserModel {
            uid: "1".to_string(),
            user_role: "admin".to_string(),
            name_last: "Doe".to_string(),
            name_first: "John".to_string(),
            email: "john.doe@example.com".to_string(),
            phone: "+1234567890".to_string(),
            region: "US".to_string(),
            realm_id: "1".to_string(),
            created_at: timestamp,
        },
        // Add more user data as needed
    ];

    // Convert user data to CSV format
    let csv_data: String = users
        .iter()
        .map(|user| {
            format!(
                "{},{},{},{},{},{},{},{},{}\n",
                user.uid,
                user.user_role,
                user.name_last,
                user.name_first,
                user.email,
                user.phone,
                user.region,
                user.realm_id,
                user.created_at,
            )
        })
        .collect();

    // Write CSV data to file
    // csv_write(&filename, csv_data)?;

    let response = serde_json::json!({
        "status": "success",
        "message": "User data exported to CSV",
    });

    Ok((StatusCode::OK, Json(response)))
}

pub async fn delete_users_from_csv(
    // Path(filename): Path<String>,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    // Delete CSV file
    // delete_csv(&filename)?;

    let response = serde_json::json!({
        "status": "success",
        "message": "CSV file deleted successfully",
    });

    Ok((StatusCode::OK, Json(response)))
}


fn to_user_response(user: &UserModel) -> UserModelResponse {
    UserModelResponse {
        uid: user.uid.clone(),
        user_role: user.user_role.clone(),
        name_last: user.name_last.clone(),
        name_first: user.name_first.clone(),
        email: user.email.clone(),
        phone: user.phone.clone(),
        region: user.region.clone(),
        realm_id: user.realm_id.clone(),
        created_at: user.created_at.naive_local(),
    }
}

pub async fn post_users_request(users: &[UserModel], data: &Arc<AppState>) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    let timestamp: chrono::DateTime<chrono::Local> = chrono::Local::now();
    let formatted_timestamp = timestamp.format("%Y-%m-%d %H:%M:%S").to_string(); // Format timestamp

    for user in users {
        let query_result = sqlx::query(r#"
            INSERT INTO user (name_last, name_first, email, phone, region, realm_id, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        "#)
        .bind(&user.name_last)
        .bind(&user.name_first)
        .bind(&user.email)
        .bind(&user.phone)
        .bind(&user.region)
        .bind(&user.realm_id)
        .bind(&formatted_timestamp) // Bind formatted timestamp
        .bind(&formatted_timestamp) // Bind formatted timestamp
        .execute(&data.db)
        .await;

        if let Err(err) = query_result {
            let error_message = format!("Error returned from database: {:?}", err);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": error_message }))));
        }
    }

    Ok(())
}