use axum::{Json, extract::State};
use derive_builder::Builder;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection};
use serde::Deserialize;
use validator::Validate;

use crate::{
    entity::{self, connections::Model},
    errors::{ApplicationError, ApplicationErrorBuilder, ApplicationErrorType},
    guards::ValidatedJson,
    routes::AppState,
};

#[derive(Builder, Debug, Deserialize, Validate)]
pub struct CreateConnectionInput {
    #[validate(range(min = 0, max = 9999999))]
    pub initiator_uid: u32,
    #[validate(range(min = 0, max = 9999999))]
    pub recipient_uid: u32,
}

// #[derive(Builder, Debug)]
// pub struct UpdateConnectionStatusInput {
//     pub id: i32,
//     pub status: String,
// }

#[axum::debug_handler]
pub async fn create_conn_handler(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<CreateConnectionInput>,
) -> Result<Json<Model>, ApplicationError> {
    let conn_model = create_conn(payload.initiator_uid, payload.recipient_uid, state.db).await?;
    Ok(Json(conn_model))
}

// TODO: Validation - 1 - make sure the users exist
// TODO: Validation - 2 - if a user already has a connection - no more additional request should be
// made
pub async fn create_conn(
    initiator_uid: u32,
    recipient_uid: u32,
    db: DatabaseConnection,
) -> eyre::Result<Model> {
    if initiator_uid == recipient_uid {
        let err = ApplicationErrorBuilder::default()
            .err_type(ApplicationErrorType::Validation)
            .message("Initiator and recipient user id cannot be same".into())
            .code(400)
            .build()?;

        return Err(eyre::Report::wrap_err(
            err.into(),
            "Failed to create connection, initiator and recipient user id cannot be same",
        ));
    }

    let connection = entity::connections::ActiveModel {
        initiator_uid: ActiveValue::Set(initiator_uid as i32),
        recipient_uid: ActiveValue::Set(recipient_uid as i32),
        ..Default::default()
    }
    .insert(&db)
    .await?;

    Ok(connection)
}

pub fn update_conn_status() {}
