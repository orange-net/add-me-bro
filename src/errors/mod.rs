use std::default::Default;

use axum::{Json, extract::rejection::JsonRejection, http::StatusCode, response::IntoResponse};
use derive_builder::Builder;
use parse_display_derive::Display;
use serde::{Deserialize, Serialize};
use validator::ValidationErrors;

#[derive(Serialize, Deserialize, Debug, Clone, Default, Display)]
pub enum ApplicationErrorType {
    Validation,
    Internal,
    #[default]
    Default,
}

#[derive(Debug, Clone, Default, Builder, Serialize, Deserialize, Display)]
#[display("ErrorType: {err_type}, ErrorCode: {code}, Message: {message}")]
pub struct ApplicationError {
    pub err_type: ApplicationErrorType,
    pub code: i16,
    pub message: String,
}

impl std::error::Error for ApplicationError {}

#[derive(Serialize)]
struct ErrorResponse {
    code: i16,
    error_type: ApplicationErrorType,
    message: String,
}

impl IntoResponse for ApplicationError {
    fn into_response(self) -> axum::response::Response {
        let status = match self.err_type {
            ApplicationErrorType::Validation => StatusCode::BAD_REQUEST,
            ApplicationErrorType::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            ApplicationErrorType::Default => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = Json(ErrorResponse {
            code: status.as_u16() as i16,
            error_type: self.err_type,
            message: self.message,
        });

        body.into_response()
    }
}

impl From<eyre::Error> for ApplicationError {
    fn from(err: eyre::Error) -> Self {
        // log the full error chain internally
        tracing::error!("{}", err);

        if let Some(app_err) = err.downcast_ref::<ApplicationError>() {
            // app_err: &ApplicationError
            return app_err.clone();
        }

        ApplicationError {
            err_type: ApplicationErrorType::Internal,
            code: 1000,
            message: "Internal server error".to_string(),
        }
    }
}

impl From<JsonRejection> for ApplicationError {
    fn from(value: JsonRejection) -> Self {
        let err_msg = match value {
            JsonRejection::JsonDataError(json_data_error) => {
                format!("Invalid JSON data: {}", json_data_error.body_text())
            }
            JsonRejection::JsonSyntaxError(json_syntax_error) => {
                format!("Invalid JSON syntax: {}", json_syntax_error.body_text())
            }
            JsonRejection::MissingJsonContentType(missing_content_type) => {
                format!(
                    "Missing or invalid Content-Type header: {}",
                    missing_content_type
                )
            }
            JsonRejection::BytesRejection(bytes_rejection) => {
                format!("Failed to read request body: {}", bytes_rejection)
            }
            _ => "Invalid JSON request".to_string(),
        };

        ApplicationError {
            code: 400,
            err_type: ApplicationErrorType::Validation,
            message: err_msg,
        }
    }
}

impl From<ValidationErrors> for ApplicationError {
    fn from(val_errors: ValidationErrors) -> Self {
        let mut err_msg = "error".to_string();

        let error_map = val_errors.field_errors();

        for v in error_map.values() {
            for i in *v {
                err_msg = i.message.clone().unwrap().to_string();
            }
        }

        ApplicationError {
            code: 400,
            err_type: ApplicationErrorType::Validation,
            message: err_msg,
        }
    }
}
