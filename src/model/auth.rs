#![allow(unused)]

use axum::http::StatusCode;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use crate::model::convert_db_to_user;
use crate::model::user::User;
use crate::repositories::user_repository::{get_user_by_email, get_user_by_username};
use crate::resources::{JWT_LIFE_SPAN, JWT_TOKKEN};

#[derive(Deserialize)]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserTokenCheck {
    pub token: String
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub username: String,
    pub email: String,
    pub role: String,
    pub token: String
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub uid: String,
    pub role: String,
    pub exp: usize,
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct DBToken {
    pub uid: String,
    pub token: String,
}

#[derive(Clone)]
pub struct AuthController { }

impl AuthController {
    pub async fn new() -> crate::Result<Self> { Ok(Self {  }) }
}

impl AuthController {
    pub async fn login(&self, login_info: LoginInfo) -> Result<User, StatusCode> {
        let mut user;
        if(login_info.username.contains("@")) {
            user = get_user_by_email(login_info.username.as_str()).await;
        } else {
            user = get_user_by_username(login_info.username.as_str()).await;
        }

        if(user.is_err()) {
            return Err(StatusCode::UNAUTHORIZED)
        }

        let warped_user = user.unwrap();
        if(!warped_user.password.eq(login_info.password.as_str())) {
            return Err(StatusCode::UNAUTHORIZED)
        }

        Ok(convert_db_to_user(warped_user).unwrap())
    }

    pub fn generate_jwt(&self, user_email: &str, user_role: &str, uid: &str, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let expiration = Utc::now() + Duration::days(*JWT_LIFE_SPAN);

        let claims = Claims {
            sub: user_email.to_string(),
            uid: uid.to_string(),
            role: user_role.to_string(),
            exp: expiration.timestamp() as usize,
        };

        encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
    }

    pub fn decode_jwt(&self, token: &str, secret: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default()
        )
    }

    pub fn is_valid(&self, claim: &Claims) -> bool {
        claim.exp > Utc::now().timestamp() as usize
    }
}
