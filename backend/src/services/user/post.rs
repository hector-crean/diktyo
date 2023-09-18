use crate::{authentication::new_session, errors::authentication::SignupError, AppState};
use bibe_models::{
    session_token::SessionToken,
    user::{CreateUser, CreateUserResponse, Role},
};

use axum::{extract::State, response::Json};
use pbkdf2::{
    password_hash::{PasswordHasher, SaltString},
    Pbkdf2,
};
use rand::prelude::*;
use rand_core::OsRng;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
struct UserRow {
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
