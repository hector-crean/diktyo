use crate::AppState;

use axum::{extract::State, response::Json};
use bibe_models::bike::Bike;
use uuid::Uuid;

use super::post::UpdateBikeStatus;

pub struct GetBike {
    pub bike_id: Uuid,
}

pub async fn get_bike(
    State(state): State<AppState>,
    Json(GetBike { bike_id }): Json<GetBike>,
) -> Result<Json<Bike>, sqlx::Error> {
    let bike = sqlx::query_as::<_, Bike>(r#"select * from "bike" where bike_id = $1"#)
        .bind(bike_id)
        .fetch_one(&state.pool)
        .await?;

    tracing::debug!("update bike status: {:?}", &bike);

    Ok(Json(bike))
}
