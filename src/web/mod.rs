#![allow(unused)]

use std::future::Future;
use axum::body::Body;
use axum::extract::{Request, State};
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use crate::Error;
use crate::model::auth::{AuthController};
use crate::model::convert_db_to_user;
use crate::model::user::User;
use crate::repositories::user_repository::{get_user_by_email};
use crate::resources::JWT_TOKKEN;

pub mod user_rest;
pub mod auth_rest;

/// Function for handle auth protected routes.
/// This routes can be used this by adding this function as layer.
/// Note that this function using jwt as auth system.
pub async fn auth_layer(
    State(auth_controller): State<AuthController>,
    headers: HeaderMap,
    request: Request<Body>,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    let authentication_token = headers.get("Authorization").ok_or(Error::UserTokenCorrupted);
    if authentication_token.is_err() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    match auth_controller.decode_jwt(authentication_token.unwrap().to_str().unwrap(), JWT_TOKKEN.as_str()) {
        Ok(token) => {
            if auth_controller.is_valid(&token.claims) {
                let user_indicator = &token.claims.sub;
                if user_indicator.contains("@") {
                    match get_user_by_email(user_indicator).await {
                        Ok(user) => {
                            let serialized_user = convert_db_to_user(user).unwrap();
                            let response = Json(serialized_user).into_response();
                            Ok(response)
                        }
                        Err(_) => {
                            let response = Response::builder()
                                .status(StatusCode::UNAUTHORIZED)
                                .body("Token not found!".into())
                                .unwrap();
                            Ok(response)
                        }
                    }
                } else {
                    let response = Response::builder()
                        .status(StatusCode::UNAUTHORIZED)
                        .body("Currently Not supported! Please use email...".into())
                        .unwrap();
                    Ok(response)
                }
            } else {
                let response = Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("Invalid token!".into())
                    .unwrap();
                Ok(response)
            }
        }
        Err(error) => {
            eprintln!("Server Internal Error: {}", error);
            Err(StatusCode::UNAUTHORIZED)
        },
    }
}