use crate::AppState;
use axum::{
    extract::State,
    response::{IntoResponse, Json, Response},
};
use bibe_models::bike::{CreateBike, UpdateBikeStatus};
use bibe_models::{
    bike::{Bike, BikeStatus},
    message::BibeMsg,
};
use http::StatusCode;
use tokio::sync::broadcast::error::SendError;

pub async fn create_bike(
    State(state): State<AppState>,
    Json(CreateBike {
        status,
        last_maintenance_date,
    }): Json<CreateBike>,
) -> Result<Json<Bike>, UpdateBikeStatusError> {
    let bike = sqlx::query_as::<_, Bike>(
        r#"insert into "bike" (status, last_maintenance_date) values ($1, $2) returning *"#,
    )
    .bind(status.unwrap_or(BikeStatus::Available) as BikeStatus)
    .bind(last_maintenance_date)
    .fetch_one(&state.pool)
    .await?;

    tracing::debug!("create bike: {:?}", bike);

    Ok(Json(bike))
}

#[derive(thiserror::Error, Debug)]
pub enum UpdateBikeStatusError {
    #[error(transparent)]
    BikeCommandSendError(#[from] SendError<BibeMsg>),
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
}

impl IntoResponse for UpdateBikeStatusError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "").into_response()
    }
}

pub async fn update_bike_status(
    State(state): State<AppState>,
    Json(UpdateBikeStatus { bike_id, status }): Json<UpdateBikeStatus>,
) -> Result<Json<Bike>, UpdateBikeStatusError> {
    let mut trans = state.pool.begin().await?;

    let resp = sqlx::query_as::<_, Bike>(
        r#"update "bike" set status = $2, updated_at = NOW() where bike_id = $1 returning *"#,
    )
    .bind(bike_id)
    .bind(status)
    .fetch_one(&mut *trans)
    .await?;

    let sent = match resp.status {
        BikeStatus::Available | BikeStatus::UnderMaintenance => state.tx.send(BibeMsg::LockBike {
            bike_id: resp.bike_id,
        }),
        BikeStatus::Rented => state.tx.send(BibeMsg::UnlockBike {
            bike_id: resp.bike_id,
        }),
    };

    match sent {
        Err(e) => {
            trans.rollback().await?;
            Err(UpdateBikeStatusError::BikeCommandSendError(e))
        }
        Ok(_) => {
            tracing::debug!("update bike status: {:?}", &resp);

            // Commit the transaction since both operations succeeded
            trans.commit().await?;

            Ok(Json(resp))
        }
    }
}
