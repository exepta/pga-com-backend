#![allow(unused)]

use axum::http::StatusCode;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use crate::model::user::User;
use crate::repositories::user_repository::{get_user_by_email, get_user_by_username};

#[derive(Deserialize)]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
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

        Ok(User {
            username: warped_user.username,
            email: warped_user.email,
            password: warped_user.password,
            role: warped_user.role,
            created_at: warped_user.created_at.to_string(),
            updated_at: warped_user.updated_at.to_string(),
        })
    }

    pub fn generate_jwt(&self, user_email: &str, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let expiration = Utc::now()
            .checked_sub_signed(Duration::hours(1))
            .expect("Timestamp invalid...")
            .timestamp();

        let claims = Claims {
            sub: user_email.to_string(),
            exp: expiration as usize,
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
