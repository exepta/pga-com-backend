#![allow(unused)]

use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Duration, Utc};

use jsonwebtoken::{encode, decode, Header, EncodingKey, TokenData, DecodingKey, Validation};

use crate::resources::JWT_TOKKEN;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub email: String
}

pub fn encode_jwt(email: String) -> Result<String, StatusCode> {
    let now: DateTime<Utc> = Utc::now();
    let expire: Duration = Duration::days(14);

    let claims: Claims = Claims {
        iat: now.timestamp() as usize,
        exp: (now + expire).timestamp() as usize,
        email
    };
    let secret = JWT_TOKKEN.as_str();

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
        .map_err(|_| { StatusCode::INTERNAL_SERVER_ERROR })
}

pub fn decode_jwt(jwt: String) -> Result<TokenData<Claims>, StatusCode> {
    let secret = JWT_TOKKEN.as_str();
    let resource: Result<TokenData<Claims>, StatusCode> = decode(&jwt, &DecodingKey::from_secret(secret.as_ref()), &Validation::default())
        .map_err(|_| { StatusCode::INTERNAL_SERVER_ERROR });

    Ok(resource?)
}