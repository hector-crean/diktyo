use chrono::{DateTime, Utc};
use sqlx::types::Uuid;

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct RentalSession {
    pub rental_session_id: Uuid,
    pub user_id: Option<Uuid>,
    pub bike_id: Option<Uuid>,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub start_location: Option<geo_types::Point<f64>>,
    pub end_location: Option<geo_types::Point<f64>>,
    pub updated_at: DateTime<Utc>,
}
