use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

// region:    --- User
#[derive(Clone, Debug, Serialize)]
pub struct User {
    id: String,
    username: String,
    email: String,
    password: String,
    role: String,
}

#[derive(Deserialize)]
pub struct UserForCreate {
    username: String,
    email: String,
    password: String,
}
// endregion: --- User

// region:  --- Model Controller
#[derive(Clone)]
pub struct ModelController {
    user_store: Arc<Mutex<Vec<Option<User>>>>
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            user_store: Arc::default(),
        })
    }
}

//CRUD
impl ModelController {
    pub async fn create_user(&self, user_fc: UserForCreate) -> Result<User> {
        let mut store = self.user_store.lock().unwrap();
        let id = Uuid::new_v4().to_string();
        let role= "admin".to_string();

        let user = User {
            id,
            username: user_fc.username,
            email: user_fc.email,
            password: user_fc.password,
            role,
        };
        store.push(Some(user.clone()));

        Ok(user)
    }

    pub async fn delete_user(&self, id: String) -> Result<User> {
        let mut store = self.user_store.lock().unwrap();

        let user = store.remove((&id).parse().unwrap()).and_then(|u| Option::from(u));

        user.ok_or(Error::UserDeleteByIdFailed { id })
    }

    pub async fn list_users(&self) -> Result<Vec<User>> {
        let store = self.user_store.lock().unwrap();
        let users = store.iter().filter_map(|u| u.clone()).collect();

        Ok(users)
    }
}

// endregion --- Model Controller
