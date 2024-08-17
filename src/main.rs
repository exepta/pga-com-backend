mod resources;
mod database;
mod repositories;
mod web;
mod model;
mod error;

pub use self::error::{Error, Result};

use axum::{Router};
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
    let user_controller = UserController::new().await;

    let router: Router = Router::new()
        .nest("/api", web::user_rest::routes(user_controller.unwrap().clone()));

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