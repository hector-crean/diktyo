use super::session_token::SessionToken;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::AsRefStr;
use uuid::Uuid;

use rand::{
    distributions::{Alphanumeric, Distribution, Standard},
    prelude::*,
};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    // #[serde(skip_serializing)]
    pub password_hash: String,
    pub role: Role,
    pub updated_at: DateTime<Utc>,
}

#[derive(
    Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, sqlx::Type, AsRefStr,
)]
#[sqlx(type_name = "role", rename_all = "lowercase")]
pub enum Role {
    User,
    Superuser,
    Admin,
    Moderator,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: Role,
}

#[derive(sqlx::FromRow)]
struct UserRow {
    pub user_id: Uuid,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct CreateUserResponse {
    pub session_token: SessionToken,
    pub user_id: Uuid,
}

impl Distribution<CreateUser> for Standard {
    fn sample<R: Rng + ?Sized>(&self, mut rng: &mut R) -> CreateUser {
        let username = (&mut rng)
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect::<String>();

        let email_radix = (&mut rng)
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect::<String>();

        let password = (&mut rng)
            .sample_iter(&Alphanumeric)
            .take(12)
            .map(char::from)
            .collect::<String>();

        CreateUser {
            username,
            email: format!("{}@r42.com", email_radix),
            password,
            role: Role::User,
        }
    }
}
