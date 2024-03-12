use std::sync::Arc;

use axum::{
    routing::{delete, get, post},
    Router,
};

use super::csv_handler::{create_new_users_from_csv, delete_users_from_csv, user_list_to_csv};

use crate::AppState;

pub fn create_csv_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/",
            post(create_new_users_from_csv)
                .get(user_list_to_csv)
                .delete(delete_users_from_csv),
        )
        // .route(
        //     "/csv/realm",
        //     post()
        //     //     .get()
        //     //     .delete(),
        // )
        .with_state(app_state)
}
