#![allow(unused)]

use crate::database::generate_pool;
use crate::Error;
use crate::model::auth::DBToken;

/// This stored the given token and bind it to the uid from the user.
pub async fn store_token(uid: &str, token: &str) -> Result<bool, Error> {
    let query = "INSERT INTO tokens (uid, token) VALUES ($1, $2)";
    let pool = generate_pool().await;

    let result = sqlx::query(query)
        .bind(uid)
        .bind(token)
        .execute(&pool)
        .await;
    if(result.is_err()) {
        let error = result.err().unwrap();
        eprintln!("SQL Exception: {}", error);
        return Err(Error::TokenStoreFailed {error: error.to_string()})
    }

    pool.close().await;
    Ok(result.is_ok())
}

/// fetch the token by the given uid.
pub async fn get_token_by_uid(uid: &str) -> Result<DBToken, Error> {
    let query = "";
    let pool = generate_pool().await;

    let result = sqlx::query_as::<_, DBToken>(query)
        .bind(uid)
        .fetch_one(&pool)
        .await;

    if(result.is_err()) {
        eprintln!("SQL Exception: {}", result.err().unwrap());
        return Err(Error::TokenFetchFailed)
    }

    Ok(result.unwrap())
}
