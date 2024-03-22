use std::sync::Arc;

use axum::{
    routing::{delete, get, post},
    Router,
};

use super::csv_handler::{create_new_users_from_csv, create_new_users_from_csv_with_file_name, delete_users_except};

use crate::AppState;

pub fn create_csv_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/",
            post(create_new_users_from_csv)
                .delete(delete_users_except),
        )
        .route(
            "/:filename",
            post(create_new_users_from_csv_with_file_name)
            //     .get()
            //     .delete(),
        )
        .with_state(app_state)
}
