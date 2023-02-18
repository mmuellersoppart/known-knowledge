use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

pub struct AppError {
    pub status_code: StatusCode,
    pub message: String,
}

impl AppError {
    pub fn new(status_code: StatusCode, message: impl Into<String>) -> Self {
        Self {
            status_code,
            message: message.into(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
                self.status_code,
            Json(ResponseMessage { message:self.message })
        ).into_response()
    }
}

#[derive(Serialize)]
struct ResponseMessage {
    message: String,
}