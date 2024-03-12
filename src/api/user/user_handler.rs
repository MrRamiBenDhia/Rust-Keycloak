use std::sync::Arc;
use axum::{
    extract::{Path, Json, State},
    http::StatusCode,
    response::IntoResponse,
};
// mod csv_manager;

use crate::{
    api::user::{user_model::UserModelResponseMessedUp, user_schema::{CreateUserSchema, UpdateUserSchema}}, csv::csv_manager::csv_manager::csv_read, AppState
};

use serde_json::json;

use super::user_model::{UserModel, UserModelResponse};

pub async fn user_list_handler(State(data): State<Arc<AppState>>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Retrieve users from the database and convert to UserModelResponse
    let users = sqlx::query_as!(
        UserModelResponseMessedUp,
        r#"SELECT * FROM user"#
    )
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "error",
                "message": format!("Database error: { }", e),
            })),
        )
    })?;

    // Convert UserModel to UserModelResponse
    let user_responses = users
        .iter()
        .map(|user| to_user_response_messedup(&user))
        .collect::<Vec<UserModelResponse>>();

    let json_response = serde_json::json!({
        "status": "ok",
        "users": user_responses,
    });

    Ok(Json(json_response))
}

pub async fn create_user_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Insert user into the database
    let query_result = sqlx::query(r#"
        INSERT INTO users (name_last, name_first, email, phone, region, realm_id)
        VALUES (?, ?, ?, ?, ?, ?)
    "#)
    .bind(body.name_last.to_string())
    .bind(body.name_first.to_string())
    .bind(body.email.to_string())
    .bind(body.phone.to_string())
    .bind(body.region.to_string())
    .bind(body.realm_id.clone())
    .execute(&data.db)
    .await;

    // Check for duplicate entry error
    if let Err(err) = &query_result {
        if err.to_string().contains("Duplicate entry") {
            let error_response = serde_json::json!({
                "status": "error",
                "message": "User with the provided email already exists",
            });
            return Err((StatusCode::CONFLICT, Json(error_response)));
        }
    }

    // Handle the result
    let user = match query_result {
        Ok(_) => {
            // Get inserted user by email
            sqlx::query_as!(UserModelResponseMessedUp, r#"SELECT * FROM user WHERE email = ?"#, body.email)
                .fetch_one(&data.db)
                .await
                .map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({
                            "status": "error",
                            "message": format!("{:?}", e),
                        })),
                    )
                })?
        }
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": format!("{:?}", err),
                })),
            ));
        }
    };

    let user_response = serde_json::json!({
        "status": "success",
        "data": serde_json::json!({
            "user": to_user_response_messedup(&user),
        }),
    });

    Ok(Json(user_response))
}

pub async fn get_user_handler(
    Path(uid): Path<String>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Retrieve user from the database based on the provided ID
    let user = sqlx::query_as!(
        UserModelResponseMessedUp,
        r#"SELECT * FROM user WHERE uid = ?"#,
        uid.to_string()
    )
    .fetch_one(&data.db)
    .await;

    // Check & response
    match user {
        Ok(user) => {
            let user_response = serde_json::json!({
                "status": "success",
                "data": serde_json::json!({
                    "user": to_user_response_messedup(&user),
                }),
            });

            Ok(Json(user_response))
        }
        Err(sqlx::Error::RowNotFound) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("User with ID: {} not found", uid),
            });
            Err((StatusCode::NOT_FOUND, Json(error_response)))
        }
        Err(e) => {
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": format!("{:?}", e),
                })),
            ))
        }
    }
}

pub async fn edit_user_handler(
    Path(uid): Path<String>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Validate user with query macro
    let query_result = sqlx::query_as!(
        UserModelResponseMessedUp,
        r#"SELECT * FROM user WHERE uid = ?"#,
        uid.to_string()
    )
    .fetch_one(&data.db)
    .await;

    // Fetch the result
    let user = match query_result {
        Ok(user) => user,
        Err(sqlx::Error::RowNotFound) => {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("User with ID: {} not found", uid),
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": format!("{:?}", e),
                })),
            ));
        }
    };

    // Update user in the database based on the provided ID
    let update_result = sqlx::query(r#"UPDATE users SET name_last = ?, name_first = ?, email = ?, phone = ?, region = ?, realm_id = ? WHERE id = ?"#)
        .bind(body.name_last.clone().unwrap_or(user.name_last.unwrap_or_else(|| {"null".to_owned()})))
        .bind(body.name_first.clone().unwrap_or(user.name_first.unwrap_or_else(|| {"null".to_owned()})))
        .bind(body.email.clone().unwrap_or(user.email.unwrap_or_else(|| {"null".to_owned()})))
        .bind(body.phone.clone().unwrap_or(user.phone.unwrap_or_else(|| {"null".to_owned()})))
        .bind(body.region.clone().unwrap_or(user.region.unwrap_or_else(|| {"null".to_owned()})))
        .bind(body.realm_id.clone().unwrap_or(user.realm_id.unwrap_or_else(|| {0}).to_string()))
        .bind(uid.to_string())
        .execute(&data.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": format!("{:?}", e),
                })),
            )
        })?;

    // Check if the user was updated successfully
    if update_result.rows_affected() == 0 {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("User with ID: {} not found", uid),
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    // Get updated user by ID
    let updated_user = sqlx::query_as!(
        UserModelResponseMessedUp,
        r#"SELECT * FROM user WHERE uid = ?"#,
        uid
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "error",
                "message": format!("{:?}", e),
            })),
        )
    })?;

    let user_response = serde_json::json!({
        "status": "success",
        "data": serde_json::json!({
            "user": to_user_response_messedup(&updated_user),
        }),
    });

    Ok(Json(user_response))
}

pub async fn delete_user_handler(
    Path(id): Path<String>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Delete user from the database based on the provided ID
    let delete_result = sqlx::query(r#"DELETE FROM users WHERE uid = ?"#)
        .bind(id.to_string())
        .execute(&data.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": format!("{:?}", e),
                })),
            )
        })?;

    // Check if the user was deleted successfully
    if delete_result.rows_affected() == 0 {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("User with ID: {} not found", id),
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let json_response = serde_json::json!({
        "status": "success",
        "message": format!("User with ID: {} deleted successfully", id),
    });

    Ok(Json(json_response))
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
        created_at: user.created_at.clone(),
    }
}

fn to_user_response_messedup(user: &UserModelResponseMessedUp) -> UserModelResponse {
    UserModelResponse {
        uid: user.uid.to_string(),
        user_role: user.user_role.clone().unwrap_or_else(|| {"null".to_owned()}),
        name_last: user.name_last.clone().unwrap_or_else(|| {"null".to_owned()}),
        name_first: user.name_first.clone().unwrap_or_else(|| {"null".to_owned()}),
        email: user.email.clone().unwrap_or_else(|| {"null".to_owned()}),
        phone: user.phone.clone().unwrap_or_else(|| {"null".to_owned()}),
        region: user.region.clone().unwrap_or_else(|| {"null".to_owned()}),
        realm_id: user.realm_id.clone().unwrap_or_else(|| {0}).to_string(),
        created_at: user.created_at.clone().to_owned(),
    }
}
