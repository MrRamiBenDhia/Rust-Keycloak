// realm_handler.rs

use std::sync::Arc;

use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{
    api::realm::realm_model::{RealmModelMessedUp, RealmModelResponse},
    AppState,
};

use super::{realm_model::RealmModel, realm_schema::{CreateRealmSchema, UpdateRealmSchema}};

pub async fn realm_list_handler(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Retrieve realms from the database and convert to RealmModelResponse
    let realms = match sqlx::query!(
        r#"SELECT id, name, description, enabled, verify_email, remember_me FROM realm"#
    )
    .fetch_all(&data.db)
    .await
    {
        Ok(rows) => rows
            .into_iter()
            .map(|row| {
                let id: String = row.id.to_string();
                let name: String = row.name.expect("REASON").to_string();
                let description: String = row.description.expect("REASON").to_string();
                let enabled: bool = row.enabled != 0;
                let verify_email: bool = row.verify_email != 0;
                let remember_me: bool = row.remember_me != 0;

                RealmModelResponse {
                    id,
                    name,
                    description,
                    enabled,
                    verify_email,
                    remember_me,
                }
            })
            .collect::<Vec<RealmModelResponse>>(),
        Err(err) => {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Database error: { }", err),
            });
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
        }
    };

    let json_response = serde_json::json!({
        "status": "ok",
        "realms": realms,
    });

    Ok(Json(json_response))
}

pub async fn create_realm_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateRealmSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Insert realm into the database
    // let id = uuid::Uuid::new_v4().to_string();
    let query_result = sqlx::query!(
        r#"INSERT INTO realm ( name, description, enabled, verify_email, remember_me) VALUES (?, ?, ?, ?, ?)"#,
        body.name.clone(),
        body.description.clone(),
        body.enabled.unwrap_or(true),
        body.verify_email.unwrap_or(false),
        body.remember_me.unwrap_or(false)
    )
    .execute(&data.db)
    .await
    .map_err(|err| {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Database error: { }", err),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    if query_result.rows_affected() == 0 {
        let error_response = serde_json::json!({
            "status": "error",
            "message": "Failed to create realm",
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    // Fetch the created realm from the database
    let realm = sqlx::query_as!(
        RealmModelMessedUp,
        r#"SELECT * FROM realm WHERE name = ?"#,
        body.name
    )
    .fetch_one(&data.db)
    .await
    .map_err(|err| {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Database error: { }", err),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let json_response = serde_json::json!({
        "status": "success",
        "data": serde_json::json!({
            "realm": to_realm_response_messedup(&realm),
        }),
    });

    Ok(Json(json_response))
}

pub async fn get_realm_handler(
    Path(id): Path<String>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Retrieve realm from the database based on the provided ID
    let realm = match sqlx::query_as!(
        RealmModelMessedUp,
        r#"SELECT * FROM realm WHERE id = ?"#,
        id
    )
    .fetch_one(&data.db)
    .await
    {
        Ok(realm) => realm,
        Err(sqlx::Error::RowNotFound) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Realm with ID: {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Database error: { }", err),
            });
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
        }
    };

    let json_response = serde_json::json!({
        "status": "success",
        "data": serde_json::json!({
            "realm": to_realm_response_messedup(&realm),
        }),
    });

    Ok(Json(json_response))
}

pub async fn edit_realm_handler(
    Path(id): Path<String>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateRealmSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Update realm in the database based on the provided ID
    let query_result = sqlx::query!(
        r#"UPDATE realm SET name = ?, description = ?, enabled = ?, verify_email = ?, remember_me = ? WHERE id = ?"#,
        body.name.unwrap_or("".to_string()),
        body.description.unwrap_or("".to_string()),
        body.enabled.unwrap_or(true),
        body.verify_email.unwrap_or(false),
        body.remember_me.unwrap_or(false),
        id
    )
    .execute(&data.db)
    .await
    .map_err(|err| {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Database error: { }", err),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    if query_result.rows_affected() == 0 {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Realm with ID: {} not found", id),
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    // Fetch the updated realm from the database
    let realm = sqlx::query_as!(
        RealmModelMessedUp,
        r#"SELECT * FROM realm WHERE id = ?"#,
        id
    )
    .fetch_one(&data.db)
    .await
    .map_err(|err| {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Database error: { }", err),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let json_response = serde_json::json!({
        "status": "success",
        "data": serde_json::json!({
            "realm": to_realm_response_messedup(&realm),
        }),
    });

    Ok(Json(json_response))
}

pub async fn delete_realm_handler(
    Path(id): Path<String>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Delete realm from the database based on the provided ID
    let query_result = sqlx::query!(
        r#"DELETE FROM realm WHERE id = ?"#,
        id
    )
    .execute(&data.db)
    .await
    .map_err(|err| {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Database error: { }", err),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    if query_result.rows_affected() == 0 {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Realm with ID: {} not found", id),
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let json_response = serde_json::json!({
        "status": "success",
        "message": format!("Realm with ID: {} deleted successfully", id),
    });

    Ok(Json(json_response))
}

// Helper function to convert RealmModel to RealmModelResponse
fn to_realm_response(realm: &RealmModel) -> RealmModelResponse {
    RealmModelResponse {
        id: realm.id.clone(),
        name: realm.name.clone(),
        description: realm.description.clone(),
        enabled: realm.enabled.clone(),
        verify_email: realm.verify_email.clone(),
        remember_me: realm.remember_me.clone(),
    }
}
// Helper function to convert RealmModel to RealmModelResponse
fn to_realm_response_messedup(realm: &RealmModelMessedUp) -> RealmModelResponse {
    RealmModelResponse {
        id: realm.id.to_string(),
        name: realm.name.clone().unwrap_or_else(|| {"null".to_owned()}),
        description: realm.description.clone().unwrap_or_else(|| {"null".to_owned()}),
        enabled: realm.enabled.is_ascii(),
        verify_email: realm.verify_email.is_ascii(),
        remember_me: realm.remember_me.is_ascii(),
    }
}
