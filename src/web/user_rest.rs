#![allow(unused)]

use axum::{Json, Router};
use axum::extract::State;
use axum::routing::post;
use crate::Error;
use crate::model::user::{User, UserController, UserForCreation};

pub fn routes(controller: UserController) -> Router {
    Router::new()
        .route("/v0/users", post(create_user).get(list_all_users))
        .with_state(controller)
}

async fn create_user(State(controller): State<UserController>, Json(user_fc): Json<UserForCreation>) -> Result<Json<User>, Error> {
    let user = controller.create(user_fc).await;
    if(user.is_err()) {
        return Err(Error::UserCreationFailed { username: "".to_string() })
    }

    Ok(Json::from(user?))
}

async fn list_all_users(State(controller): State<UserController>) -> Result<Json<Vec<User>>, Error> {
    let users = controller.list_all_users().await;
    if(users.is_err()) {
        return Err(Error::UserListCannotBeFetch);
    }

    Ok(Json(users?))
}