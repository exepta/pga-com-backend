use axum::extract::{FromRef, Path, State};
use axum::{Json, Router};
use axum::routing::{delete, post};
use crate::model::{ModelController, User, UserForCreate};
use crate::Result;

#[derive(Clone, FromRef)]
struct AppState {
    mc: ModelController,
}

pub fn routes(mc: ModelController) -> Router {
    let app_state = AppState {mc};
    Router::new()
        .route("/v0/users", post(create_user).get(list_users))
        .route("/v0/users/del/:id", delete(delete_user))
        .with_state(app_state)
}

async fn create_user(State(mc): State<ModelController>, Json(user_fc): Json<UserForCreate>) -> Result<Json<User>> {
    println!("->> {:<12} - create_user", "HANDLER");

    let user = mc.create_user(user_fc).await?;

    Ok(Json(user))
}

async fn list_users(State(mc): State<ModelController>) -> Result<Json<Vec<User>>> {
    println!("->> {:<12} - list_users", "HANDLER");

    let users = mc.list_users().await?;

    Ok(Json(users))
}

async fn delete_user(State(mc): State<ModelController>, Path(id): Path<String>) -> Result<Json<User>> {
    println!("->> {:<12} - delete_user", "HANDLER");

    let del_user = mc.delete_user(id).await?;

    Ok(Json(del_user))
}