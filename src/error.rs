use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::components;
use crate::layouts::default::DefaultLayout;

#[allow(dead_code)]
pub type PageResult<T> = core::result::Result<T, ErrorPage>;

#[derive(Debug)]
pub enum ErrorPage {
    NotFound(Option<String>),
    InternalServerError(Option<String>),
}

impl IntoResponse for ErrorPage {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::NotFound(message) => (StatusCode::NOT_FOUND, message.unwrap_or("404 Not Found".into())),
            Self::InternalServerError(message) => (StatusCode::INTERNAL_SERVER_ERROR, message.unwrap_or("500 Internal Server Error".into())),
        };

        DefaultLayout::new(
            &message,
            components::error(&message, None),
        )
            .set_status(status)
            .into_response()
    }
}

pub type ApiResult<T> = core::result::Result<T, ApiError>;

#[derive(Debug)]
pub enum ApiError {
    #[allow(dead_code)]
    DatabaseError(sqlx::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            Self::DatabaseError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "DATABASE_ERROR").into_response()
            }
        }
    }
}

#[derive(Debug)]
pub enum ServiceError {
    DatabaseError(String, sqlx::Error),
}
