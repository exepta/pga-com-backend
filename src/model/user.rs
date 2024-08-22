#![allow(unused)]

use std::hint::black_box;
use crate::{Error};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::model::convert_db_to_user;
use crate::repositories;
use crate::repositories::user_repository::{create_db_user, delete_user, get_user_by_email, get_users, get_users_with_attrib, DBUser, get_user_by_username};

#[derive(Clone, Debug, Serialize)]
pub struct User {
    pub uid: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub birthday: Option<String>,
    pub avatar_path: Option<String>,
    pub banner_path: Option<String>,
    pub configurations: Option<String>, //Separator char ';'
    pub created_at: String,
    pub updated_at: String,
}

impl Default for User {
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
            created_at: String::new(),
            updated_at: String::new(),
        }
    }
}

#[derive(Deserialize)]
pub struct UserForCreation {
    pub username: String,
    pub email: String,
    pub password: String,
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

        let uuid = Uuid::new_v4().to_string();
        println!("UUID={}", uuid);

        create_db_user(&DBUser {
            uid: uuid,
            username: user_fc.username,
            email: email.to_string(),
            password: user_fc.password,
            role: "member".to_string(),
            created_at: Default::default(),
            updated_at: Default::default(),
            ..Default::default()
        }).await;

        let db_user_raw = get_user_by_email(&email.as_str()).await;
        if(db_user_raw.is_err()) {
            return Err(Error::UserCreationFailed { username: email.to_string() })
        }

        let db_user = db_user_raw.unwrap();

        Ok(User {
            uid: db_user.uid,
            username: db_user.username,
            email: db_user.email,
            password: db_user.password,
            role: db_user.role,
            created_at: db_user.created_at.to_string(),
            updated_at: db_user.updated_at.to_string(),
            ..Default::default()
        })
    }

    pub async fn delete(&self, email: &str) -> crate::Result<bool> {
        let state = delete_user(email).await.unwrap();
        Ok(state)
    }

    pub async fn get_user_by_name_or_email(&self, data: &str) -> Result<User, Error> {
        let mut user :DBUser;
        if(data.contains("@")) {
            let result = get_user_by_email(&data).await;
            if(result.is_err()) {
                return Err(Error::UserNotFound {email: data.to_string()})
            }
            user = result.unwrap();
        } else {
            let result = get_user_by_username(&data).await;
            if(result.is_err()) {
                return Err(Error::UserNotFound {email: data.to_string()})
            }
            user = result.unwrap();
        }

        Ok(convert_db_to_user(user).unwrap())
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
            convert_db_to_user(db_user).unwrap()
        }).collect();
        Ok(users)
    }
}
