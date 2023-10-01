use std::error::Error;

use aide::OperationIo;
use axum::{http::StatusCode, response::IntoResponse};
use invidious::InvidiousError;
use schemars::JsonSchema;
use serde::Serialize;
use serde_json::Value;
use uuid::Uuid;

/// A default error response for most API errors.
#[derive(Debug, Serialize, JsonSchema, OperationIo)]
pub struct AppError {
    /// An error message.
    pub error: String,
    /// A unique error ID.
    pub error_id: Uuid,
    #[serde(skip)]
    pub status: StatusCode,
    /// Optional Additional error details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_details: Option<Value>,
}

impl AppError {
    pub fn new(error: &str) -> Self {
        Self {
            error: error.to_string(),
            error_id: Uuid::new_v4(),
            status: StatusCode::BAD_REQUEST,
            error_details: None,
        }
    }

    pub fn with_status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }

    pub fn with_details(mut self, details: Value) -> Self {
        self.error_details = Some(details);
        self
    }
}

impl From<InvidiousError> for AppError {
    fn from(value: InvidiousError) -> Self {
        Self::new("Invidious api error")
            .with_status(StatusCode::INTERNAL_SERVER_ERROR)
            .with_details(Value::String(value.to_string()))
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = self.status;
        let mut res = axum::Json(self).into_response();
        *res.status_mut() = status;
        res
    }
}
