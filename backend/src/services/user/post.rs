use crate::{
    authentication::{new_session, SessionToken},
    errors::authentication::SignupError,
    AppState,
};
use bibe_models::user::Role;

use axum::{extract::State, response::Json};
use pbkdf2::{
    password_hash::{PasswordHasher, SaltString},
    Pbkdf2,
};
use rand::{
    distributions::{Alphanumeric, Distribution, Standard},
    prelude::*,
};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

pub async fn create_user(
    State(state): State<AppState>,
    Json(CreateUser {
        username,
        email,
        password,
        role,
    }): Json<CreateUser>,
) -> Result<Json<CreateUserResponse>, SignupError> {
    let salt = SaltString::generate(&mut OsRng);

    // Hash password to PHC string ($pbkdf2-sha256$...)

    let password_hash = if let Ok(password) = Pbkdf2.hash_password(password.as_bytes(), &salt) {
        password.to_string()
    } else {
        return Err(SignupError::InvalidPassword);
    };

    let UserRow { user_id} = sqlx::query_as::<_, UserRow>(
        r#"insert into "user"(username, email, password_hash, role) values ($1, $2, $3, $4) returning user_id"#,
    )
    .bind(username)
    .bind(email)
    .bind(password_hash)
    .bind(role)
    .fetch_one(&state.pool)
    .await
    .expect("Unable to insert a user");

    tracing::debug!("create user: {:?}", user_id);

    let session_token = new_session(&state.pool, state.random, user_id).await;

    Ok(Json(CreateUserResponse {
        session_token,
        user_id,
    }))
}

// utils
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
