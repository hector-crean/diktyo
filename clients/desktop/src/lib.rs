pub mod errors;

use bibe_models::{
    bike::{Bike, BikeStatus, CreateBike, UpdateBikeStatus},
    user::{CreateUser, CreateUserResponse},
};
use errors::BibeDesktopClientError;
use serde_json::json;

const ENDPOINT_ROOT: &str = "127.0.0.1:1690";

pub async fn create_bike() -> Result<(), BibeDesktopClientError> {
    let client = reqwest::Client::new();

    // Create a `CreateUser` instance
    let bike: CreateBike = CreateBike {
        status: Some(BikeStatus::UnderMaintenance),
        last_maintenance_date: None,
    };

    let resp = client
        .post(format!("http://{}/v1/api/bike", ENDPOINT_ROOT))
        .json(&json!(bike))
        .send()
        .await?
        .json::<Bike>()
        .await?;

    Ok(())
}

pub async fn update_bike() -> Result<(), BibeDesktopClientError> {
    let client = reqwest::Client::new();

    // Create a `CreateUser` instance
    let update: UpdateBikeStatus = UpdateBikeStatus {
        bike_id: uuid::uuid!("77fbaa72-d293-400b-ba57-e1203fc0a989"),
        status: BikeStatus::Rented,
    };

    let resp = client
        .post(format!(
            "http://{}/v1/api/bike/77fbaa72-d293-400b-ba57-e1203fc0a989",
            ENDPOINT_ROOT
        ))
        .json(&json!(update))
        .send()
        .await?
        .json::<Bike>()
        .await?;

    Ok(())
}
