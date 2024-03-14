use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::AppState;

use super::fibonacci_handler::get_fibonacci_handler;



pub fn create_router(app_state: Arc<AppState>) -> Router {

    Router::new().route("/:number", get(get_fibonacci_handler))
    .with_state(app_state)
    
}