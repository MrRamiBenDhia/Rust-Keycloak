use crate::AppState;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;

use super::crypto_logic::CryptoHandler;
pub async fn handle_crypto_sha_test(
    Path(number_iter): Path<usize>,

    State(_app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let start_time = std::time::Instant::now(); // Start measuring time

    let data = "some_data_to_hash"; // Replace with actual data to hash
    let mut data_table: Vec<String> = Vec::new();

    // CryptoHandler::stress_test(number, data);
    // CryptoHandler::stress_test(number, data);

    let mut counter: usize = 0;
    loop {
        let together = format!("{data}{counter}");
        let _x = CryptoHandler::hash_sha256(&together);

        data_table.push(_x);

        if counter > number_iter {
            break;
        }
        counter += 1;
    }

    let elapsed_time = start_time.elapsed();
    let elapsed_millis = elapsed_time.as_millis();
    println!("Function executed in {} milliseconds", elapsed_millis);

    let json_response = serde_json::json!({
            "status": "success",
            "count": number_iter,
            "result": data_table[0],
           "elapsed_time": format!("{:.3} seconds",elapsed_millis as f32 /1000.0),
           "elapsed_millis": elapsed_millis,
    });

    Ok((StatusCode::OK, Json(json_response)))
}
pub async fn handle_crypto_md5_test(
    Path(number_iter): Path<usize>,

    State(_app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let start_time = std::time::Instant::now(); // Start measuring time

    let data = "some_data_to_hash"; // Replace with actual data to hash
    let mut data_table: Vec<String> = Vec::new();

    let mut counter: usize = 0;
    loop {
        let together = format!("{data}{counter}");
        let _x = CryptoHandler::hash_md5(&together);

        data_table.push(_x);

        if counter > number_iter {
            break;
        }
        counter += 1;
    }

    let elapsed_time = start_time.elapsed();
    let elapsed_millis = elapsed_time.as_millis();
    println!("Function executed in {} milliseconds", elapsed_millis);

    let json_response = serde_json::json!({
            "status": "success",
            "count": number_iter,
            "result": data_table[0],
           "elapsed_time": format!("{:.3} seconds",elapsed_millis as f32 /1000.0),
           "elapsed_millis": elapsed_millis,
    });

    Ok((StatusCode::OK, Json(json_response)))
}
