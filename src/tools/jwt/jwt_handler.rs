use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::AppState;

use super::jwt_sign_verify::jwt_sign_verify_main;

pub async fn jwt_sign_verify_operations(
    Path(number): Path<u32>,
    State(_data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let duration = jwt_sign_verify_main(number);

    let elapsed_millis = duration.as_millis();

    let json_response = serde_json::json!({
            "status": "success",
            "count": number,
            // "result": result,//! result here if u want to see it
           "elapsed_time": format!("{:.3} seconds",elapsed_millis as f32 /1000.0),
           "elapsed_millis": elapsed_millis,
    });

    Ok((StatusCode::OK, Json(json_response)))

    // Ok((StatusCode::OK, Json(("Duration: ", duration.as_secs_f64()))))
}
