use axum::{
    extract::{Json, Path, Query, State}, http::{Response, StatusCode}, response::IntoResponse, routing::post, Router
};
use chrono::{Local, NaiveDateTime};
use std::sync::Arc;
use tokio::sync::Mutex;

use super::csv_manager::{
    csv_deserialize, csv_read, csv_write, delete_csv,
};
use crate::{
    api::user::user_model::UserModel,
    AppState,
};
pub async fn create_new_users_from_csv(
    // Path(filename): Path<String>,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    //! bismilah !
    
    // Read CSV file
    // let csv_data = csv_read(&filename)?;
    // let _csv_data_result = csv_read("misc/MOCK_DATA.csv");
    let _result_vec = csv_deserialize("misc/Rust_User_3.csv");











    println!("Result: {:?}",_result_vec);
    // let csv_data = match csv_data_result {
    //     Ok(data) => data,
    //     Err(err) => {
    //         // Handle the error (e.g., log it or return an error response)
    //         let error_response: (StatusCode, Json<serde_json::Value>) = map_csv_error_to_response(err, StatusCode::BAD_REQUEST);
    //         return Err(error_response);
    //     }
    //     };


    // // Process CSV data and insert into the database (add your logic here)
    // let users: Vec<UserModel> = csv_data
    //     .iter()
    //     .map(|record| {
    //         // Convert CSV record to UserModel (add your conversion logic here)
    //         // UserModel {
    //         //     uid: record[0].clone(),
    //         //     user_role: record[1].clone(),
    //         //     name_last: record[2].clone(),
    //         //     name_first: record[3].clone(),
    //         //     email: record[4].clone(),
    //         //     phone: record[5].clone(),
    //         //     region: record[6].clone(),
    //         //     realm_id: record[7].clone(),
    //         //     // created_at: record[8].clone(),
    //         // }
    //     })
    //     .collect();

    let response = serde_json::json!({
        "status": "success",
        "message": "Users created successfully from CSV",
    });

    Ok((StatusCode::OK, Json(response)))
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