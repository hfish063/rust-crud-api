mod handler;
mod model;
mod route;

use std::sync::Arc;

use axum::{response::IntoResponse, Json};

use dotenv::dotenv;
use tokio::net::TcpListener;

use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

use crate::route::create_router;

pub struct AppState {
    db: MySqlPool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("REST API Service");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("Connection to DB successful");
            pool
        }
        Err(err) => {
            println!("Failed to connect to database: {:?}", err);
            std::process::exit(1);
        }
    };

    let app = create_router(Arc::new(AppState { db: pool.clone() }));

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

pub async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "API Services";

    let json_response = serde_json::json!({
        "status": "OK",
        "message": MESSAGE
    });

    Json(json_response)
}
