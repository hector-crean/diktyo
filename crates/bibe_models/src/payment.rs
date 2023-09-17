use chrono::{DateTime, Utc};
use sqlx::types::Uuid;

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct Payment {
    pub payment_id: Uuid,
    pub rental_session_id: Uuid,
    pub amount: f64,
    pub payment_method: String,
    pub payment_status: String,
    pub payment_date: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
