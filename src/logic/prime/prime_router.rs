use std::sync::Arc;

use axum::{
    routing::get,
    Router,
};

use crate::AppState;

use super::prime_handler::get_prime_handler;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/:number", get(get_prime_handler))
        .with_state(app_state)
}
