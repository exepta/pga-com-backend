use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFail,

    UserListCannotBeFetch,
    UserTokenExpired,
    UserTokenCorrupted,
    UserUidIsEmpty,
    UserNotFound {email: String},
    UserWasFoundByEmail {email: String},
    UserDeleteByIdFailed {id: String},
    UserCreationFailed {username: String},
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}