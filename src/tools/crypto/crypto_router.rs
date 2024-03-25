use crate::AppState;
use std::sync::Arc;

use axum::{
    routing::{get},
    Router,
};

use super::crypto_handler::{handle_crypto_md5_test,handle_crypto_sha_test};

pub fn create_crypto_router(app_state: Arc<AppState>) -> Router {
    let router = Router::new();
    router
    .route("/sha256/:num_iterations", get(handle_crypto_sha_test))
        .route("/md5/:num_iterations", get(handle_crypto_md5_test))
        .with_state(app_state)
    
}
