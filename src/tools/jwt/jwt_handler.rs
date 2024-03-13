use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use crate::AppState;

use super::jwt_sign_verify::jwt_sign_verify_main;

pub async fn jwt_sign_verify_operations(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let duration = jwt_sign_verify_main();

    Ok((StatusCode::OK, Json(("Duration: ", duration.as_secs_f64()))))

}
