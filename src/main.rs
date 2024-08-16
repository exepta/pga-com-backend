#![allow(unused)]

pub use self::error::{Error, Result};

use std::fmt::format;
use std::net::SocketAddr;
use axum::response::{Html, Response};
use axum::{middleware, Router, ServiceExt};
use axum::routing::get;
use tower_cookies::CookieManagerLayer;
use crate::model::ModelController;

mod error;
mod web;
mod model;

#[tokio::main]
async fn main() -> Result<()> {
    let mc = ModelController::new().await?;

    let router = Router::new()
        .nest("/api", web::route_login::routes())
        .nest("/api", web::route_user::routes(mc.clone()))
        .layer(middleware::map_response(master_response_mapper))
        .layer(CookieManagerLayer::new());

    let listener = tokio::net::TcpListener::bind(format!("localhost:{}", 8090))
        .await.unwrap();

    println!("Successfully bound on port [ {:?} ]\n", listener.local_addr());
    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn master_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - master_response_mapper", "RES_MAPPER");

    println!();
    res
}
