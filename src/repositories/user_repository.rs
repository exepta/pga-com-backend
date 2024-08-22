#![allow(unused)]

use std::fmt::format;
use sqlx::Error;
use sqlx::types::chrono::NaiveDateTime;
use crate::database;
use crate::database::generate_pool;
use crate::model::user::User;

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct DBUser {
    pub uid: String,
    pub username: String,
    pub password: String,
    pub email: String,
    pub role: String,
    pub birthday: Option<String>,
    pub avatar_path: Option<String>,
    pub banner_path: Option<String>,
    pub configurations: Option<String>, //Separator char ';'
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Default for DBUser {
    fn default() -> Self {
        Self {
            uid: String::new(),
            username: String::new(),
            email: String::new(),
            password: String::new(),
            role: String::from("member"),
            birthday: None,
            avatar_path: None,
            banner_path: None,
            configurations: None,
            created_at: NaiveDateTime::default(),
            updated_at: NaiveDateTime::default(),
        }
    }
}

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
        Err(sqlx::Error::RowNotFound) => {
            println!("No user found with email: {}", email);
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
        Err(sqlx::Error::RowNotFound) => {
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