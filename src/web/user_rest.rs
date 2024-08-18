#![allow(unused)]

use std::ptr::addr_eq;
use axum::{Json, Router};
use axum::extract::{Path, State};
use axum::routing::{get, post};
use crate::Error;
use crate::model::user::{User, UserController, UserForCreation};

pub fn routes(controller: UserController) -> Router {
    Router::new()
        .route("/v0/users", post(create_user).get(list_all_users))
        .route("/v0/users/:attrib/:value", get(list_attrib_users))
        .with_state(controller)
}

/// Create new entry for users in the users table.
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

async fn list_attrib_users(State(controller): State<UserController>,
                           Path((attrib, value)): Path<(String, String)>)
    -> Result<Json<Vec<User>>, Error> {
    let users = controller.list_attrib_users(attrib.as_str(), value.as_str()).await;
    if(users.is_err()) {
        return Err(Error::UserListCannotBeFetch);
    }

    Ok(Json(users?))
}