#![allow(unused)]

use std::hint::black_box;
use crate::{Error};
use serde::{Deserialize, Serialize};
use crate::repositories;
use crate::repositories::user_repository::{create_db_user, delete_user, get_user_by_email, get_users, get_users_with_attrib, DBUser};

#[derive(Clone, Debug, Serialize)]
pub struct User {
    username: String,
    email: String,
    password: String,
    role: String,
    created_at: String,
    updated_at: String,
}

#[derive(Deserialize)]
pub struct UserForCreation {
    username: String,
    email: String,
    password: String,
}

#[derive(Clone)]
pub struct UserController { }

impl UserController {
    pub async fn new() -> crate::Result<Self> {
        Ok(Self {  })
    }
}

impl UserController {
    pub async fn create(&self, user_fc: UserForCreation) -> Result<User, Error> {
        let email = &user_fc.email;
        if(get_user_by_email(email.as_str()).await.is_ok()) {
            return Err(Error::UserWasFoundByEmail {email: email.to_string()})
        }

        create_db_user(&DBUser {
            username: user_fc.username,
            email: email.to_string(),
            password: user_fc.password,
            role: "member".to_string(),
            created_at: Default::default(),
            updated_at: Default::default(),
        }).await;

        let db_user = get_user_by_email(email.as_str()).await.unwrap();

        Ok(User {
            username: db_user.username,
            email: db_user.email,
            password: db_user.password,
            role: db_user.role,
            created_at: db_user.created_at.to_string(),
            updated_at: db_user.updated_at.to_string(),
        })
    }

    pub async fn delete(&self, email: &str) -> crate::Result<bool> {
        let state = delete_user(email).await.unwrap();
        Ok(state)
    }

    pub async fn list_all_users(&self) -> Result<Vec<User>, Error> {
        let users = get_users().await;
        if(users.is_err()) {
            return Err(Error::UserListCannotBeFetch);
        }

        let db_users = users.unwrap();

        Ok(Self::convert_user_to_vec(db_users)?)
    }

    pub async fn list_attrib_users(&self, attrib: &str, value: &str) -> Result<Vec<User>, Error> {
        let users = get_users_with_attrib(attrib, value).await;
        if(users.is_err()) {
            return Err(Error::UserListCannotBeFetch);
        }

        let db_users = users.unwrap();

        Ok(Self::convert_user_to_vec(db_users)?)
    }

    fn convert_user_to_vec(db_users: Vec<DBUser>) -> crate::Result<Vec<User>> {
        let users = db_users.into_iter().map(|db_user| {
            User {
                username: db_user.username,
                email: db_user.email,
                password: db_user.password,
                role: db_user.role,
                created_at: db_user.created_at.to_string(),
                updated_at: db_user.updated_at.to_string(),
            }
        }).collect();
        Ok(users)
    }
}
