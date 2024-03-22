use crate::AppState;
use axum::{
    extract::{Json, Path, Query, State},
    http::{Response, StatusCode},
    response::IntoResponse,
    routing::post,
    Router,
};
use std::sync::Arc;

use super::crypto_logic::CryptoHandler;
pub async fn handle_crypto_sha_test(
    Path(numberIter): Path<usize>,

    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let start_time = std::time::Instant::now(); // Start measuring time

    let data = "some_data_to_hash"; // Replace with actual data to hash
    let mut data_table: Vec<String> = Vec::new();

    // CryptoHandler::stress_test(number, data);
    // CryptoHandler::stress_test(number, data);

    let mut counter: usize = 0;
    loop {
        let together = format!("{data}{counter}");
        let x = CryptoHandler::hash_sha256(&together);

        // data_table.push(x);

        if counter > numberIter {
            break;
        }
        counter += 1;
    }

    let elapsed_time = start_time.elapsed();
    let elapsed_millis = elapsed_time.as_millis();
    println!("Function executed in {} milliseconds", elapsed_millis);

    let response = serde_json::json!({
        "status": "success",
        "message": "Crypto Is Donee",
        "Data":data_table,

        "elapsed_time": elapsed_millis,
    });

    Ok((StatusCode::OK, Json(response)))
}
pub async fn handle_crypto_md5_test(
    Path(numberIter): Path<usize>,

    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let start_time = std::time::Instant::now(); // Start measuring time

    let data = "some_data_to_hash"; // Replace with actual data to hash
    let mut data_table: Vec<String> = Vec::new();

    let mut counter: usize = 0;
    loop {
        let together = format!("{data}{counter}");
        let x = CryptoHandler::hash_md5(&together);

        // data_table.push(x);

        if counter > numberIter {
            break;
        }
        counter += 1;
    }

    let elapsed_time = start_time.elapsed();
    let elapsed_millis = elapsed_time.as_millis();
    println!("Function executed in {} milliseconds", elapsed_millis);

    let response = serde_json::json!({
        "status": "success",
        "message": "Crypto Is Donee",
        "Data":data_table,

        "elapsed_time": elapsed_millis,
    });

    Ok((StatusCode::OK, Json(response)))
}
