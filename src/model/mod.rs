use crate::Error;
use crate::model::user::User;
use crate::repositories::user_repository::DBUser;

pub mod user;
pub mod auth;

pub fn convert_db_to_user(db_user: DBUser) -> Result<User, Error> {
    let user = User {
        uid: db_user.uid,
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

    if user.uid.is_empty() {
        return Err(Error::UserUidIsEmpty)
    }

    Ok(user)
}