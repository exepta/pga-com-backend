#![allow(unused)]

use axum::{debug_handler, Json, Router};
use axum::extract::{FromRef, State};
use axum::http::{StatusCode};
use axum::http::header::{CONTENT_TYPE, SET_COOKIE};
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use chrono::{Duration, TimeDelta};
use jsonwebtoken::{encode, EncodingKey, Header};
use tower_cookies::{cookie, Cookie};
use tower_cookies::cookie::SameSite;
use crate::Error;
use crate::model::auth::{AuthController, Claims, LoginInfo, LoginResponse};
use crate::model::user::{User, UserController, UserForCreation};
use crate::repositories::user_repository::{create_db_user, get_user_by_email, DBUser};
use crate::resources::JWT_TOKKEN;
use crate::util::pass_hash::hash_password;

#[derive(Clone, FromRef)]
struct AppState {
    uc: UserController,
    ac: AuthController
}

pub fn routes(uc: UserController, ac: AuthController) -> Router {
    let app_state = AppState { uc, ac };
    Router::new()
        .route("/v0/auth/login", post(login_user))
        .route("/v0/auth/register", post(register_user))
        .with_state(app_state)
}

async fn register_user(State(controller): State<UserController>, Json(reg_user): Json<UserForCreation>) -> Result<Json<User>, StatusCode> {
    let mut data = reg_user;
    let hash = hash_password(data.password);
    data.password = hash;
    
    match controller.create(UserForCreation {
        username: data.username,
        password: data.password,
        email: data.email
    }).await {
        Ok(user) => {
            println!("Successfully created user");
            Ok(Json::from(user))
        }
        Err(error) => {
            eprintln!("Failed to register because [{:?}]", error);
            Err(StatusCode::UNAUTHORIZED)
        }
    }

}

async fn login_user(State(controller): State<AuthController>, Json(login_user): Json<LoginInfo>) -> Result<Json<LoginResponse>, StatusCode> {
    let mut data = login_user;
    let hash = hash_password(data.password);
    data.password = hash;

    let raw_login = controller.login(data).await;
    if(raw_login.is_err()) {
        return Err(StatusCode::UNAUTHORIZED)
    }

    let user = raw_login.unwrap();
    let token = controller.generate_jwt(&user.email.as_str(), JWT_TOKKEN.as_str());
    if(token.is_err()) {
        return Err(StatusCode::INTERNAL_SERVER_ERROR)
    }

    Ok(Json(LoginResponse {
        username: user.username,
        email: user.email,
        token: token.unwrap(),
    }))
}