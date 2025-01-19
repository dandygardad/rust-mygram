mod models;
mod schema;
mod repositories;

use axum::response::{Html, IntoResponse};
use axum::{Json, Router};
use dotenv::dotenv;
use axum::routing::{get, get_service};
use sqlx::postgres::{PgPool, PgPoolOptions};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use std::sync::Arc;

// Create state
pub struct AppState {
    db: PgPool
}

#[tokio::main]
async fn main() {
    println!("\n\n📷 Rust API MyGram 📷");
    println!("Taken from Hacktiv8 Go Final Project converted to Rust\n");

    // Config and DB
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("❌ DATABASE_URL must set to .env");
    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url).await
    {
        Ok(pool) => {
            println!("✅ Connection to postgres db is successfully");
            pool
        }
        Err(err) => {
            println!("❌ Failed to connect to database, error: {:?}", err);
            std::process::exit(1)
        }
    };
    // End of config and DB

    // Router
    let router = Router::new()
        .route("/", get(index))
        .route("/test", get(test_api))
        .with_state(Arc::new(AppState { db: pool.clone() }))
        .fallback_service(static_files());

    fn static_files() -> Router {
        Router::new().fallback_service(get_service(ServeDir::new("./public")))
    }
    // End of Router

    // Listener and serve
    let listener = TcpListener::bind("127.0.0.1:8001").await.unwrap();
    let extracted_addr = listener.local_addr().unwrap();
    println!("ℹ️ Will be served into {}:{}", extracted_addr.ip(), extracted_addr.port());
    axum::serve(listener, router.into_make_service()).await.unwrap()
    // End of listener and serve
}

// Handlers for default
async fn index() -> impl IntoResponse {
    Html("<h1>You are not allowed to visit this!</h1>")
}
async fn test_api() -> impl IntoResponse {
    let json_response = serde_json::json!({
            "status": "success",
            "message": "hello_world"
        });

    Json(json_response)
}