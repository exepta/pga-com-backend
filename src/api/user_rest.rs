use actix_web::{
    get,
    post,
    put,
    error::ResponseError,
    web::Path,
    web::Json,
    web::Data,
    HttpResponse,
    http::{header::ContentType, StatusCode}
};

use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct UserIdentifier {
    id :String,
}

#[get("/user/{id}")]
pub async fn get_user(id :Path<UserIdentifier>) -> Json<String> {
    return Json(id.into_inner().id);
}