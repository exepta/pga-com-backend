mod resources;
mod database;
mod repositories;
mod web;
mod model;
mod error;
mod util;

pub use self::error::{Error, Result};

use axum::{Router};
use tower_http::cors::{Any, CorsLayer};
use crate::model::auth::AuthController;
use crate::model::user::UserController;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting Backend Server...");
    database::connect_test().await;
    create_default_tables().await;
    println!("Connected to Postgres 17!");

    initialize("localhost", 8090).await;

    Ok(())
}

async fn initialize(host: &str, port: i32) {
    let user_controller = UserController::new().await.unwrap();
    let auth_controller = AuthController::new().await.unwrap();

    let cros = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any);

    let router: Router = Router::new()
        .nest("/api", web::user_rest::routes(user_controller.clone(), auth_controller.clone()))
        .nest("/api", web::auth_rest::routes(user_controller.clone(), auth_controller.clone())
            .layer(cros));

    println!("Try to bind Server on Port [ {} ]", port);
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();

    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}

async fn create_default_tables() {
    database::create_table("./migrations/0001_user_table.sql").await;
}