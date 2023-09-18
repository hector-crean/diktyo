use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct Bike {
    pub bike_id: Uuid,
    pub status: BikeStatus,
    // pub location: Option<geo_types::Point<f64>>,
    pub last_maintenance_date: Option<chrono::NaiveDate>,
    pub updated_at: DateTime<Utc>,
}

#[derive(
    Debug, Serialize, Deserialize, PartialEq, Clone, strum::EnumString, strum::Display, sqlx::Type,
)]
#[sqlx(rename_all = "snake_case", type_name = "bike_status")]
pub enum BikeStatus {
    #[strum(serialize = "available")]
    Available,
    #[strum(serialize = "rented")]
    Rented,
    #[strum(serialize = "under_maintenance")]
    UnderMaintenance,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateBike {
    pub status: Option<BikeStatus>,
    pub last_maintenance_date: Option<chrono::NaiveDate>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBike {
    pub bike_id: Uuid,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateBikeStatus {
    pub bike_id: Uuid,
    pub status: BikeStatus,
}
