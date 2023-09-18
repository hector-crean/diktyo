use crate::{Random, USER_COOKIE_NAME};
use bibe_models::{session_token::SessionToken, user::User};

use rand_core::RngCore;
use sqlx::{Pool, Postgres};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Clone)]
pub(crate) struct AuthState(Option<(SessionToken, Option<User>, Pool<Postgres>)>);

impl AuthState {
    pub fn logged_in(&self) -> bool {
        self.0.is_some()
    }
    pub async fn get_user(&mut self) -> Option<&User> {
        let (session_token, store, database) = self.0.as_mut()?;

        if store.is_none() {
            const QUERY: &str =
                "SELECT user_id, username, password_hash, email, role, updated_at FROM users JOIN session ON user_id = id WHERE session_token = $1;";

            let user: Option<User> = sqlx::query_as(QUERY)
                .bind(&session_token.into_database_value())
                .fetch_optional(&*database)
                .await
                .expect("user query failed");

            if let Some(user) = user {
                *store = Some(user);
            }
        }
        store.as_ref()
    }
}

pub(crate) async fn new_session(
    database: &Pool<Postgres>,
    random: Random,
    user_id: Uuid,
) -> SessionToken {
    const QUERY: &str = "INSERT INTO session (session_token, user_id) VALUES ($1, $2);";

    let session_token = SessionToken::generate_new(random);

    let _result = sqlx::query(QUERY)
        .bind(&session_token.into_database_value())
        .bind(user_id)
        .execute(database)
        .await
        .unwrap();

    session_token
}

pub(crate) async fn auth<B>(
    mut req: http::Request<B>,
    next: axum::middleware::Next<B>,
    database: Pool<Postgres>,
) -> axum::response::Response {
    let session_token = req
        .headers()
        .get_all("Cookie")
        .iter()
        .filter_map(|cookie| {
            cookie
                .to_str()
                .ok()
                .and_then(|cookie| cookie.parse::<cookie::Cookie>().ok())
        })
        .find_map(|cookie| {
            (cookie.name() == USER_COOKIE_NAME).then(move || cookie.value().to_owned())
        })
        .and_then(|cookie_value| cookie_value.parse::<SessionToken>().ok());

    req.extensions_mut()
        .insert(AuthState(session_token.map(|v| (v, None, database))));

    next.run(req).await
}
