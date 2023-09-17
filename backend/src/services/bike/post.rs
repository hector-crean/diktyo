use crate::{
    authentication::{new_session, SessionToken},
    errors::authentication::SignupError,
    services::bike::get::get_bike,
    AppState,
};
use bibe_models::{
    bike::{Bike, BikeStatus},
    message::BibeMsg,
    user::Role,
};
use http::StatusCode;
use sqlx::Executor;

use axum::{
    extract::State,
    response::{IntoResponse, Json, Response},
};
use pbkdf2::{
    password_hash::{PasswordHasher, SaltString},
    Pbkdf2,
};
use rand::{
    distributions::{Alphanumeric, Distribution, Standard},
    prelude::*,
};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast::error::SendError;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateBikeStatus {
    pub bike_id: Uuid,
    pub status: BikeStatus,
}

// utils

#[derive(thiserror::Error, Debug)]
pub enum UpdateBikeStatusError {
    #[error(transparent)]
    BikeCommandSendError(#[from] SendError<BibeMsg>),
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
}

impl IntoResponse for UpdateBikeStatusError {
    fn into_response(self) -> Response {
        match self {
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "").into_response(),
        }
    }
}

pub async fn update_bike_status(
    State(state): State<AppState>,
    Json(UpdateBikeStatus { bike_id, status }): Json<UpdateBikeStatus>,
) -> Result<Json<Bike>, UpdateBikeStatusError> {
    let mut trans = state.pool.begin().await?;

    let dispenser_response = sqlx::query_as::<_, Bike>(
        r#"
        INSERT INTO bike_statuses (bike_id, status)
        VALUES ($1, $2)
        ON CONFLICT (bike_id)
        DO UPDATE SET status = EXCLUDED.status
        returning *
        "#,
    )
    .bind(bike_id)
    .bind(status.to_string())
    .fetch_one(&mut *trans)
    .await?;

    let sent = match dispenser_response.status.as_str() {
        "Locked" => state.tx.send(BibeMsg::StopDrinkDispensing {
            dispenser_id: dispenser_response.bike_id,
        }),
        "Unlocked" => state.tx.send(BibeMsg::StartDrinkDispensing {
            dispenser_id: dispenser_response.bike_id,
        }),
        _ => state.tx.send(BibeMsg::StopDrinkDispensing {
            dispenser_id: dispenser_response.bike_id,
        }),
    };

    match sent {
        Err(e) => {
            trans.rollback().await?;
            Err(UpdateBikeStatusError::BikeCommandSendError(e))
        }
        Ok(_) => {
            tracing::debug!("update bike status: {:?}", &dispenser_response);

            // Commit the transaction since both operations succeeded
            trans.commit().await?;

            Ok(Json(dispenser_response))
        }
    }
}
