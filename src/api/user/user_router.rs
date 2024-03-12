// route.rs

use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    api::user::user_handler::{
        create_user_handler, delete_user_handler, edit_user_handler, get_user_handler,
        user_list_handler,
    },
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", post(create_user_handler))
        .route("/", get(user_list_handler))
        .route(
            "/:uid",
            get(get_user_handler)
                .patch(edit_user_handler)
                .delete(delete_user_handler),
        )
        .with_state(app_state)
}
