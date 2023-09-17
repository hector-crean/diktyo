use chrono::{DateTime, Utc};


use strum::AsRefStr;
use uuid::Uuid;

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
