use axum::{Json, Router};
use axum::routing::post;
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use crate::{Error, Result};
use crate::web::AUTH_TOKEN;

pub fn routes() -> Router {
    Router::new().route("/api/v0/login", post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {

    //Todo: make here correct work flows like data base...
    if(payload.username != "exepta" || payload.password != "123456") {
        return Err(Error::LoginFail);
    }

    cookies.add(Cookie::new(AUTH_TOKEN, "user-id.expire.sign"));

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));
    Ok(body)

}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}