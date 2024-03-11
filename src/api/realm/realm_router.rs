// route.rs

use std::sync::Arc;

use axum::{
    routing::{get, post, patch, delete},
    Router,
};

use crate::{
    api::realm::realm_handler::{
        // create_realm_handler, delete_realm_handler, edit_realm_handler, get_realm_handler,
         realm_list_handler,
    },
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        // .route("/api/healthcheck", get(health_check_handler))
        .route("/", get(realm_list_handler))
        // .route("/realm", post(create_realm_handler))
        // .route(
        //     "/realm/:id",
        //     get(get_realm_handler)
        //         .patch(edit_realm_handler)
        //         .delete(delete_realm_handler),
        // )
        .with_state(app_state)
}
