use sqlx::types::Uuid;

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct Session {
    pub session_token: Vec<u8>,
    pub user_id: Uuid,
}
