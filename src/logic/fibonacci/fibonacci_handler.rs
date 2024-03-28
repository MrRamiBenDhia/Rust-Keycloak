use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
};

use super::fibonacci_seq::{fibonacci_seq, fibonacci_seq_none_recursive};
pub async fn get_fibonacci_handler(
    Path(number): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let n: u32 = number.parse::<u32>().unwrap();

    let start_time = std::time::Instant::now(); // Start measuring time

    let result = fibonacci_seq(n);

    let elapsed_time = start_time.elapsed(); // Measure elapsed time
    let elapsed_millis = elapsed_time.as_millis();

    let json_response = serde_json::json!({
            "status": "success",
            "count": n,
            "result": result,
           "elapsed_time": format!("{:.3} seconds",elapsed_millis as f32 /1000.0),
           "elapsed_millis": elapsed_millis,
    });

    Ok((StatusCode::OK, Json(json_response)))

    // return Err((StatusCode::NOT_FOUND, Json(error_response)));
    // (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
}
pub async fn get_fibonacci_none_recursive_handler(
    Path(number): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let n: u32 = number.parse::<u32>().unwrap();

    let start_time = std::time::Instant::now(); // Start measuring time

    let result = fibonacci_seq_none_recursive(n);

    let elapsed_time = start_time.elapsed(); // Measure elapsed time
    let elapsed_millis = elapsed_time.as_millis();

    let json_response = serde_json::json!({
            "status": "success",
            "count": n,
            "result": result,
           "elapsed_time": format!("{:.3} seconds",elapsed_millis as f32 /1000.0),
           "elapsed_millis": elapsed_millis,
    });

    Ok((StatusCode::OK, Json(json_response)))

    // return Err((StatusCode::NOT_FOUND, Json(error_response)));
    // (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
}
