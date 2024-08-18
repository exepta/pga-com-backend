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


#[debug_handler]
async fn login_user(State(controller): State<AuthController>, Json(login_user): Json<LoginInfo>) -> impl IntoResponse {
    let mut data = login_user;
    let hash = hash_password(data.password);
    data.password = hash;

    let user = controller.login(LoginInfo {
        username: data.username,
        password: data.password
    }).await.unwrap();

    let json_response = Json(user.clone());

    let mut cookie = Cookie::new("token", controller.generate_jwt(user.clone().email.as_str(), JWT_TOKKEN.as_str()).unwrap());
    cookie.set_path("/");
    cookie.set_max_age(cookie::time::Duration::hours(1));
    cookie.set_same_site(SameSite::None);
    cookie.set_http_only(true);

    (StatusCode::OK, [(CONTENT_TYPE, "application/json"), (axum::http::header::SET_COOKIE, cookie.to_string().as_str())], json_response).into_response()
}