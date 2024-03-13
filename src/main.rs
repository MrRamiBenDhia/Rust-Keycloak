pub mod api {
    pub(crate) mod user {
        mod user_handler;
        pub(crate) mod user_model;
        pub(crate) mod user_router;
        mod user_schema;
    }

    pub(crate) mod realm {
        mod realm_handler;
        mod realm_model;
        pub(crate) mod realm_router;
        mod realm_schema;
    }
}

pub mod tools {
pub mod csv {
    pub mod csv_handler;
    pub(crate) mod csv_manager;
    pub mod csv_router;
}
}

use std::sync::Arc;

use axum::http::{header::CONTENT_TYPE, Method};

use dotenv::dotenv;
use tokio::net::TcpListener;

use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

use api::{
    realm::realm_router::create_router as create_realm_router,
    // client::client_router::create_router as create_client_router,
    user::user_router::create_router as create_user_router,
};

use tower_http::cors::{Any, CorsLayer};

use crate::tools::csv::csv_router::create_csv_router;

pub struct AppState {
    db: MySqlPool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("🌟 REST API Service 🌟");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must set");

    println!("~~~~~~~~~~");
    println!("{}", database_url);
    println!("~~~~~~~~~~");

    let pool = match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("❌ Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    // let client_router = create_client_router(Arc::new(AppState { db: pool.clone() })).layer(cors);
    let realm_router =
        create_realm_router(Arc::new(AppState { db: pool.clone() })).layer(cors.clone());
    let user_router =
        create_user_router(Arc::new(AppState { db: pool.clone() })).layer(cors.clone());
    let csv_router = 
        create_csv_router(Arc::new(AppState { db: pool.clone() })).layer(cors.clone());

    let app = axum::Router::new()
        .nest("/csv", csv_router)
        .nest("/realm", realm_router)
        .nest("/user", user_router);
    // .layer(cors);

    println!("✅ Server started successfully at 0.0.0.0:8000");

    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
