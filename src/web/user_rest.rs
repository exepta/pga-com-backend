#![allow(unused)]

use std::future::Future;
use std::ptr::addr_eq;
use axum::{Json, Router};
use axum::extract::{FromRef, Path, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum_extra::TypedHeader;
use jsonwebtoken::TokenData;
use tower_cookies::Cookie;
use crate::Error;
use crate::model::auth::{AuthController, Claims};
use crate::model::user::{User, UserController, UserForCreation};
use crate::repositories::user_repository::{DBUser, get_user_by_email};
use crate::resources::JWT_TOKKEN;
use crate::web::auth_layer;

#[derive(Clone, FromRef)]
struct AppState {
    uc: UserController,
    ac: AuthController
}

pub fn routes(uc: UserController, ac: AuthController) -> Router {
    let app_state = AppState { uc, ac };
    Router::new()
        .route("/v0/users", post(create_user).get(list_all_users))
        .route("/v0/users/:attrib/:value", get(list_attrib_users))
        .route("/v0/users/:data", get(user_by_name_or_email))
        .layer(axum::middleware::from_fn_with_state(app_state.clone(), auth_layer))
        .with_state(app_state.clone())
}

/// Create new entry for users in the users table.
/// TODO: This function is only usable as admin!
async fn create_user(State(controller): State<UserController>, Json(user_fc): Json<UserForCreation>) -> Result<Json<User>, Error> {
    let user = controller.create(user_fc).await;
    if(user.is_err()) {
        return Err(Error::UserCreationFailed { username: "?".to_string() })
    }

    Ok(Json::from(user?))
}

/// Delete a user from the database.
async fn delete_user(State(controller): State<UserController>, Path(email): Path<String>) -> Result<bool, Error> {
    let deleted = controller.delete(email.as_str()).await;
    if(deleted.is_err()) {
        return Err(Error::UserNotFound { email })
    }

    Ok(deleted?)
}

/// Get the complete user list as Vec<User>.
async fn list_all_users(State(controller): State<UserController>) -> Result<Json<Vec<User>>, Error> {
    let users = controller.list_all_users().await;
    if(users.is_err()) {
        return Err(Error::UserListCannotBeFetch);
    }

    Ok(Json(users?))
}

/// Get users from list by filter over attribs and his values.
async fn list_attrib_users(State(controller): State<UserController>,
                           Path((attrib, value)): Path<(String, String)>)
    -> Result<Json<Vec<User>>, Error> {
    let users = controller.list_attrib_users(attrib.as_str(), value.as_str()).await;
    if(users.is_err()) {
        return Err(Error::UserListCannotBeFetch);
    }

    Ok(Json(users?))
}

/// Get one user directly by his name or email.
async fn user_by_name_or_email(State(controller): State<UserController>, Path(data): Path<String>) -> Result<Json<User>, StatusCode> {
    let user = controller.get_user_by_name_or_email(data.as_str()).await;
    if(user.is_err()) {
        return Err(StatusCode::NOT_FOUND)
    }

    Ok(Json(user.unwrap()))
}