#![allow(unused)]

use std::fmt::format;
use sqlx::Error;
use sqlx::types::chrono::NaiveDateTime;
use crate::database;
use crate::database::generate_pool;
use crate::model::user::{DBUser, User};

/// Create a new user and save him into users database.
pub async fn create_db_user(db_user: &DBUser) {
    let query = "INSERT INTO users (uid, username, password, email, role, birthday, avatar_path, banner_path, configurations) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)";
    let pool = generate_pool().await;

    let create = sqlx::query(query)
        .bind(&db_user.uid)
        .bind(&db_user.username)
        .bind(&db_user.password)
        .bind(&db_user.email)
        .bind(&db_user.role)
        .bind(&db_user.birthday)
        .bind(&db_user.avatar_path)
        .bind(&db_user.banner_path)
        .bind(&db_user.configurations)
        .execute(&pool)
        .await;
    if(create.is_err()) {
        eprintln!("SQL Creation failed! [{}]", create.err().unwrap());
    } else {
        println!("Insert new user [ {} ]", db_user.username);
    }

    pool.close().await;
}

/// Update user by uid with attrib and value.
pub async fn update_user(uid: &str, attrib: &str, value: &str) {
    let query = format!("UPDATE users SET {} = $1 WHERE uid = $2", attrib);
    let pool = generate_pool().await;

    let result = sqlx::query(&query)
        .bind(value)
        .bind(uid)
        .execute(&pool)
        .await;

    if(result.is_err()) {
        eprintln!("SQL Update failed! [{}]", result.err().unwrap());
    } else {
        println!("Successfully updated user with uid {}", uid);
    }

    pool.close().await;
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

    let result = sqlx::query_as::<_, DBUser>(
        "SELECT * FROM users WHERE email = $1"
    )
        .bind(email)
        .fetch_one(&pool)
        .await;

    pool.close().await;

    match result {
        Ok(user) => Ok(user),
        Err(Error::RowNotFound) => {
            println!("No user found with email: {}", email);
            Err(Error::RowNotFound)
        }
        Err(e) => {
            println!("Error fetching user: {:?}", e);
            Err(e)
        }
    }
}

/// Get the user by UID
pub async fn get_user_by_uid(uid: &str) -> Result<DBUser, Error> {
    let pool = generate_pool().await;

    let result = sqlx::query_as::<_, DBUser>(
        "SELECT * FROM users WHERE uid = $1"
    )
        .bind(uid)
        .fetch_one(&pool)
        .await;

    pool.close().await;

    match result {
        Ok(user) => Ok(user),
        Err(Error::RowNotFound) => {
            println!("No user found with uid: {}", uid);
            Err(Error::RowNotFound)
        }
        Err(e) => {
            println!("Error fetching user: {:?}", e);
            Err(e)
        }
    }
}

/// Get the user by username.
pub async fn get_user_by_username(username: &str) -> Result<DBUser, Error> {
    let pool = generate_pool().await;

    let result = sqlx::query_as::<_, DBUser>(
        "SELECT * FROM users WHERE username = $1"
    )
        .bind(username)
        .fetch_one(&pool)
        .await;

    pool.close().await;

    match result {
        Ok(user) => Ok(user),
        Err(Error::RowNotFound) => {
            println!("No user found with username: {}", username);
            Err(Error::RowNotFound)
        }
        Err(e) => {
            println!("Error fetching user: {:?}", e);
            Err(e)
        }
    }
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