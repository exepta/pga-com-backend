use crate::Error;
use crate::model::user::{DBUser, User};

pub mod user;
pub mod auth;

pub fn convert_db_to_user(db_user: DBUser) -> Result<User, Error> {
    let user = User {
        id: db_user.id,
        username: db_user.username,
        email: db_user.email,
        password: db_user.password,
        role: db_user.role,
        birthday: db_user.birthday,
        avatar_path: db_user.avatar_path,
        banner_path: db_user.banner_path,
        configurations: db_user.configurations,
        created_at: db_user.created_at.to_string(),
        updated_at: db_user.updated_at.to_string(),
    };

    if user.id.is_empty() {
        return Err(Error::UserUidIsEmpty)
    }

    Ok(user)
}