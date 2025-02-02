use std::convert::From;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Debug, Clone, Default, Deserialize, Serialize, sqlx::FromRow, sqlx::Type)]
pub struct User {
    pub user_id: Uuid,
    pub email: String,
    pub username: String,
    pub password_hash: String, // convert this to secrecy secret.
    pub is_validated: bool,
    pub github: Option<String>,
    pub website: Option<String>,
    pub bio: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, sqlx::FromRow, sqlx::Type)]
pub struct SafeUser {
    pub user_id: Uuid,
    pub email: String,
    pub username: String,
    pub is_validated: bool,
    pub github: Option<String>,
    pub website: Option<String>,
    pub bio: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

impl From<User> for SafeUser {
    fn from(user: User) -> Self {
        SafeUser {
            user_id: user.user_id,
            email: user.email,
            username: user.username,
            is_validated: user.is_validated,
            github: user.github,
            website: user.website,
            bio: user.bio,
            first_name: user.first_name,
            last_name: user.last_name,
        }
    }
}
