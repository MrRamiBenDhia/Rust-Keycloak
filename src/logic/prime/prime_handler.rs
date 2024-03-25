

use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
};



use super::prime_seq::nth_prime;

pub async fn get_prime_handler(
    Path(number): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let n: u32 = number.parse::<u32>().unwrap();
    let start_time = std::time::Instant::now(); // Start measuring time


    if n > 1_000_000 {
        let error_message =
            format!("Error number limit excided, Try again with numbers less then 1 000 000 ");

        return Err((
            StatusCode::LOCKED,
            Json(serde_json::json!({ "error": error_message })),
        ));
    }
    let result = nth_prime(n);

    let elapsed_time = start_time.elapsed(); // Measure elapsed time
    let elapsed_millis = elapsed_time.as_millis();

    let json_response = serde_json::json!({
        "status": "success",
        "data": serde_json::json!({
            "count": result.len(),
            // "result": result,//! result here if u want to see it
           "elapsed_time": elapsed_millis/1000,
           "elapsed_millis": elapsed_millis,
        }),
    });

    Ok(Json(json_response))

    // (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
}
