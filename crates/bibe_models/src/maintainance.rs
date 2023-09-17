use chrono::{DateTime, Utc};
use sqlx::types::Uuid;

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct Maintenance {
    pub maintenance_id: Uuid,
    pub bike_id: Option<Uuid>,
    pub maintenance_type: Option<String>,
    pub maintenance_status: String,
    pub maintenance_date: Option<chrono::NaiveDate>,
    pub notes: Option<String>,
    pub updated_at: DateTime<Utc>,
}
