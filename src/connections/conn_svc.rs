use std::cmp::{max, min};

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use derive_builder::Builder;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{self, Set},
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use serde::Deserialize;
use serde_json::{Value, json};
use validator::{Validate, ValidationError};

use crate::{
    entity::{
        self,
        connections::{self, ActiveModel, Model},
    },
    errors::{ApplicationError, ApplicationErrorBuilder, ApplicationErrorType},
    guards::ValidatedJson,
    routes::AppState,
};

#[derive(Builder, Debug, Deserialize, Validate)]
pub struct CreateConnectionInput {
    #[validate(range(min = 0, max = 9999999))]
    pub initiator_uid: i32,
    #[validate(range(min = 0, max = 9999999))]
    pub recipient_uid: i32,
}

#[derive(Builder, Debug, Deserialize, Validate)]
pub struct UpdateConnectionInput {
    #[validate(custom(function = "validate_status"))]
    pub status: String,
}

fn validate_status(status: &str) -> Result<(), ValidationError> {
    const ALLOWED: &[&str] = &[
        "pending", "accepted", "rejected", "Pending", "Accepted", "Rejected",
    ];

    if ALLOWED.contains(&status) {
        Ok(())
    } else {
        let mut err = ValidationError::new("invalid_status");
        err.message = Some("status must be one of: pending, accepted, rejected".into());
        Err(err)
    }
}

// #[derive(Builder, Debug)]
// pub struct UpdateConnectionStatusInput {
//     pub id: i32,
//     pub status: String,
// }

pub async fn get_connection_by_users(
    initiator_uid: Option<i32>,
    recipient_uid: Option<i32>,
    db: DatabaseConnection,
) -> eyre::Result<Option<Model>> {
    let mut query = entity::connections::Entity::find();

    if let Some(uid) = initiator_uid {
        query = query.filter(connections::Column::InitiatorUid.eq(uid));
    };

    if let Some(uid) = recipient_uid {
        query = query.filter(connections::Column::RecipientUid.eq(uid));
    };

    let model = query.one(&db).await?;
    Ok(model)
}

pub fn generate_connection_key(uid_1: i32, uid_2: i32) -> String {
    let delimiter = "::";

    let min_id = min(uid_1, uid_2);
    let max_id = max(uid_1, uid_2);

    format!("{}{}{}", min_id, delimiter, max_id)
}

pub async fn users_have_conn_req(
    initiator_uid: i32,
    recipient_uid: i32,
    db: &DatabaseConnection,
) -> eyre::Result<Option<Model>> {
    let conn = entity::connections::Entity::find()
        .filter(
            connections::Column::ConnectionKey
                .eq(generate_connection_key(initiator_uid, recipient_uid)),
        )
        .one(db)
        .await?;

    Ok(conn)
}

pub async fn get_connection_by_id_or_fail(
    conn_id: i32,
    db: &DatabaseConnection,
) -> Result<connections::Model, ApplicationError> {
    let conn = connections::Entity::find_by_id(conn_id).one(db).await?;

    if let Some(val) = conn {
        return Ok(val);
    } else {
        let err = ApplicationError {
            code: StatusCode::NOT_FOUND.as_u16(),
            err_type: ApplicationErrorType::ResourceNotFound,
            message: format!("Connection with id {} not found", conn_id),
        };

        return Err(err);
    }
}

pub async fn create_conn(
    initiator_uid: i32,
    recipient_uid: i32,
    db: DatabaseConnection,
) -> eyre::Result<Model> {
    if initiator_uid == recipient_uid {
        let err = ApplicationErrorBuilder::default()
            .err_type(ApplicationErrorType::Validation)
            .message("Initiator and recipient user id cannot be same".into())
            .code(StatusCode::BAD_REQUEST.as_u16())
            .build()?;

        return Err(eyre::Report::wrap_err(
            err.into(),
            "Failed to create connection, initiator and recipient user id cannot be same",
        ));
    }

    let initiator = entity::users::Entity::find_by_id(initiator_uid)
        .one(&db)
        .await?;

    let recipient = entity::users::Entity::find_by_id(recipient_uid)
        .one(&db)
        .await?;

    if let Some(existing_conn) = users_have_conn_req(initiator_uid, recipient_uid, &db).await? {
        return Ok(existing_conn);
    }

    if initiator.is_none() || recipient.is_none() {
        let err_msg =
            "Invalid recipient or initiator of the connection request. Make sure they are valid";
        let err = ApplicationError {
            code: StatusCode::BAD_REQUEST.as_u16(),
            err_type: ApplicationErrorType::Validation,
            message: err_msg.into(),
        };

        return Err(eyre::Report::wrap_err(err.into(), err_msg));
    }

    let connection = entity::connections::ActiveModel {
        initiator_uid: ActiveValue::Set(initiator_uid),
        recipient_uid: ActiveValue::Set(recipient_uid),
        ..Default::default()
    }
    .insert(&db)
    .await?;

    Ok(connection)
}

pub async fn delete_connection_by_id(id: i32, db: &DatabaseConnection) -> eyre::Result<bool> {
    let del_response = connections::Entity::delete_by_id(id).exec(db).await?;

    if del_response.rows_affected > 0 {
        return Ok(true);
    }

    Ok(false)
}

#[axum::debug_handler]
pub async fn create_conn_handler(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<CreateConnectionInput>,
) -> Result<Json<Model>, ApplicationError> {
    let conn_model = create_conn(payload.initiator_uid, payload.recipient_uid, state.db).await?;
    Ok(Json(conn_model))
}

#[derive(Deserialize, Debug)]
pub struct RequestById {
    id: i32,
}

pub async fn delete_conn_handler(
    State(state): State<AppState>,
    Path(conn_id): Path<i32>,
) -> Result<Json<Value>, ApplicationError> {
    let _ = get_connection_by_id_or_fail(conn_id, &state.db);

    let deleted = delete_connection_by_id(conn_id, &state.db).await?;
    Ok(Json(json!({"success": deleted, "deleted": deleted })))
}

pub async fn patch_conn_handler(
    State(state): State<AppState>,
    Path(conn_id): Path<i32>,
    ValidatedJson(payload): ValidatedJson<UpdateConnectionInput>,
) -> Result<Json<Model>, ApplicationError> {
    let conn = get_connection_by_id_or_fail(conn_id, &state.db).await?;
    let mut conn: connections::ActiveModel = conn.into();
    conn.status = Set(payload.status);
    let conn = conn.update(&state.db).await?;

    Ok(Json(conn))
}
