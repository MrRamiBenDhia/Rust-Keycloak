use std::sync::Arc;

use axum::{
    routing::{get},
    Router,
};

use crate::AppState;

use super::jwt_handler::jwt_sign_verify_operations;
pub fn create_jwt_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/:number",get(jwt_sign_verify_operations))
           
        .with_state(app_state)
}
