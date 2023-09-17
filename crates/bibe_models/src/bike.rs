use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct Bike {
    pub bike_id: Uuid,
    pub status: String,
    // pub location: Option<geo_types::Point<f64>>,
    pub last_maintenance_date: Option<chrono::NaiveDate>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, strum::EnumString, strum::Display)]
pub enum BikeStatus {
    #[strum(serialize = "Locked")]
    Locked,
    #[strum(serialize = "Unlocked")]
    Unlocked,
    #[strum(serialize = "Unknown")]
    Unknown,
}
