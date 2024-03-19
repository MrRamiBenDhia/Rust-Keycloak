use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::AppState;

use super::fibonacci_handler::{get_fibonacci_handler, get_fibonacci_none_recursive_handler};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/:number",
            get(get_fibonacci_handler)
            .post(get_fibonacci_none_recursive_handler),
        )
        .route("/nr/:number", get(get_fibonacci_none_recursive_handler))
        .with_state(app_state)
}
