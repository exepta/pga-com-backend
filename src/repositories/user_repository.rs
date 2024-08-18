#![allow(unused)]

use std::fmt::format;
use sqlx::Error;
use sqlx::types::chrono::NaiveDateTime;
use crate::database;
use crate::database::generate_pool;

#[derive(sqlx::FromRow, Debug)]
pub struct DBUser {
    pub username: String,
    pub password: String,
    pub email: String,
    pub role: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Create a new user and save him into users database.
pub async fn create_db_user(db_user: &DBUser) {
    let query = "INSERT INTO users (username, password, email, role) VALUES ($1, $2, $3, $4)";
    let pool = generate_pool().await;

    sqlx::query(query)
        .bind(&db_user.username)
        .bind(&db_user.password)
        .bind(&db_user.email)
        .bind(&db_user.role)
        .execute(&pool)
        .await;

    pool.close().await;
    println!("Insert new user [ {} ]", db_user.username);
}

/// Remove a user from the user's database.
pub async fn delete_user(email: &str) -> Result<bool, Error> {
    let pool = generate_pool().await;

    let state = sqlx::query("DELETE FROM users WHERE email = $1").bind(email)
        .execute(&pool)
        .await.is_ok();

    pool.close().await;
    Ok(state)
}

/// Get the user by email
pub async fn get_user_by_email(email: &str) -> Result<DBUser, Error> {
    let pool = generate_pool().await;

    let user = sqlx::query_as::<_, DBUser>(
        "SELECT * FROM users WHERE email = $1"
    ).bind(email).fetch_one(&pool).await?;

    pool.close().await;
    Ok(user)
}

/// Get the user by username.
pub async fn get_user_by_username(username: &str) -> Result<DBUser, Error> {
    let pool = generate_pool().await;

    let user = sqlx::query_as::<_, DBUser>(
        "SELECT * FROM users WHERE username = $1"
    ).bind(username).fetch_one(&pool).await?;

    pool.close().await;
    Ok(user)
}

/// Get a list of all users.
pub async fn get_users() -> Result<Vec<DBUser>, Error> {
    let pool = generate_pool().await;

    let users = sqlx::query_as::<_, DBUser>(
        "SELECT * FROM users"
    ).fetch_all(&pool).await?;

    pool.close().await;
    Ok(users)
}

/// Get a list of users which have the same attributes.
pub async fn get_users_with_attrib(attrib: &str, value: &str) -> Result<Vec<DBUser>, Error> {
    let pool = generate_pool().await;

    let users = sqlx::query_as::<_, DBUser>(
        format!("SELECT * FROM users WHERE {} = $1", attrib).as_str()
    ).bind(value)
        .fetch_all(&pool).await?;

    pool.close().await;
    Ok(users)
}