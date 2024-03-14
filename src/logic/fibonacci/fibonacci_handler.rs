use std::sync::Arc;

use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::AppState;

use super::fibonacci_seq::fibonacci_seq;
pub async fn get_fibonacci_handler(
    Path(number): Path<String>,
    // Json(body): Json<UpdateRealmSchema>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {


    let n: u32 = number.parse::<u32>().unwrap();

    let result = fibonacci_seq(n);

    let json_response = serde_json::json!({
        "status": "success",
        "data": serde_json::json!({
            "result": result,
        }),
    });

    Ok(Json(json_response))

    // return Err((StatusCode::NOT_FOUND, Json(error_response)));
    // (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
}
