use aide::OperationIo;
use axum::{http::StatusCode, response::IntoResponse};
use invidious::InvidiousError;
use schemars::JsonSchema;
use serde::Serialize;
use serde_json::{json, Value};
use std::error::Error;
use uuid::Uuid;

pub type Response<T> = Result<T, RespError>;

#[derive(thiserror::Error, Debug, OperationIo)]
pub enum RespError {
    /// Return `401 Unauthorized`
    #[error("authentication required")]
    Unauthorized,

    /// Return `403 Forbidden`
    #[error("user may not perform that action")]
    Forbidden,

    /// Return `404 Not Found`
    #[error("request path not found")]
    NotFound,

    /// Return `400 Bad Request`
    #[error("invalid request")]
    BadRequest {
        summary: &'static str,
        details: serde_json::Value,
    },

    // #[error("an error occurred with the database")]
    // Sqlx(#[from] sqlx::Error),
    #[error("an error occurred with internal Invidious API")]
    Invidious(#[from] InvidiousError),

    #[error("an internal server error occurred")]
    Anyhow(#[from] anyhow::Error),
}

impl RespError {
    pub fn bad_request(summary: &'static str, details: serde_json::Value) -> Self {
        Self::BadRequest { summary, details }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for RespError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::BadRequest { summary, details } => {
                return (
                    StatusCode::BAD_REQUEST,
                    axum::Json(json!({"summary": summary, "details": details})),
                )
                    .into_response();
            }

            // Self::Sqlx(ref e) => {
            //     tracing::error!("SQLx error: {:?}", e);
            // }
            Self::Anyhow(ref e) => {
                tracing::error!("Generic error: {:?}", e);
            }
            _ => (),
        }

        (self.status_code(), self.to_string()).into_response()
    }
}

// /// A default error response for most API errors.
// #[derive(Debug, Serialize, JsonSchema, OperationIo)]
// pub struct RespError {
//     /// An error message.
//     pub error: String,
//     /// A unique error ID.
//     pub error_id: Uuid,
//     #[serde(skip)]
//     pub status: StatusCode,
//     /// Optional Additional error details.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub error_details: Option<Value>,
// }

// impl RespError {
//     pub fn new(error: &str) -> Self {
//         Self {
//             error: error.to_string(),
//             error_id: Uuid::new_v4(),
//             status: StatusCode::BAD_REQUEST,
//             error_details: None,
//         }
//     }

//     pub fn with_status(mut self, status: StatusCode) -> Self {
//         self.status = status;
//         self
//     }

//     pub fn with_details(mut self, details: Value) -> Self {
//         self.error_details = Some(details);
//         self
//     }
// }

// impl From<InvidiousError> for RespError {
//     fn from(value: InvidiousError) -> Self {
//         Self::new("Invidious api error")
//             .with_status(StatusCode::INTERNAL_SERVER_ERROR)
//             .with_details(Value::String(value.to_string()))
//     }
// }

// impl IntoResponse for RespError {
//     fn into_response(self) -> axum::response::Response {
//         let status = self.status;
//         let mut res = axum::Json(self).into_response();
//         *res.status_mut() = status;
//         res
//     }
// }
