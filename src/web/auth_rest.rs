#![allow(unused)]

use axum::{debug_handler, Json, Router};
use axum::extract::{FromRef, State};
use axum::http::{HeaderMap, StatusCode};
use axum::http::header::{CONTENT_TYPE, SET_COOKIE};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum_extra::headers::{Authorization, HeaderMapExt};
use chrono::{Duration, TimeDelta};
use jsonwebtoken::{encode, EncodingKey, Header};
use tower_cookies::{cookie, Cookie};
use tower_cookies::cookie::SameSite;
use crate::Error;
use crate::model::auth::{AuthController, Claims, LoginInfo, LoginResponse, UserTokenCheck};
use crate::model::convert_db_to_user;
use crate::model::user::{User, UserController, UserForCreation};
use crate::repositories::user_repository::{create_db_user, get_user_by_email, DBUser, get_user_by_uid};
use crate::resources::JWT_TOKKEN;
use crate::util::pass_hash::hash_password;
use crate::web::check_header_role;

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
        .route("/v0/auth/session_check", get(check_user_session))
        .with_state(app_state)
}

async fn register_user(State(controller): State<UserController>, Json(reg_user): Json<UserForCreation>) -> StatusCode {
    let mut data = reg_user.clone();
    let hash = hash_password(data.password);
    data.password = hash;

    if(get_user_by_email(reg_user.clone().email.as_str()).await.is_ok()) {
        return StatusCode::FOUND
    }

    match controller.create(UserForCreation {
        ..data
    }).await {
        Ok(user) => {
            println!("Successfully created user [{:?}]", user);
            StatusCode::OK
        }
        Err(error) => {
            eprintln!("Register failed [{:?}]", error);
            StatusCode::INTERNAL_SERVER_ERROR
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

    let user = raw_login?;
    let token = controller.generate_jwt(&user.email.as_str(), &user.role.as_str(), &user.uid, JWT_TOKKEN.as_str());
    if(token.is_err()) {
        return Err(StatusCode::INTERNAL_SERVER_ERROR)
    }

    Ok(Json(LoginResponse {
        username: user.username,
        email: user.email,
        role: user.role,
        token: token.unwrap(),
    }))
}

async fn check_user_session(State(controller): State<AuthController>, header: HeaderMap) -> Result<Json<User>, StatusCode> {
    let token_data = check_header_role(controller, header, "-".to_string());
    if(token_data.is_err()) {
        return Err(token_data.err().unwrap())
    }

    let raw_db_user = get_user_by_uid(token_data?.claims.uid.as_str()).await;
    if(raw_db_user.is_err()) {
        return Err(StatusCode::INTERNAL_SERVER_ERROR)
    }

    let db_user = raw_db_user.unwrap();

    Ok(Json(convert_db_to_user(db_user).unwrap()))
}