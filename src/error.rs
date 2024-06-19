use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[allow(dead_code)]
pub type PageResult<T> = core::result::Result<T, ErrorPage>;

#[derive(Debug)]
pub enum ErrorPage {
    NotFound(String),
    InternalServerError(String),
}

impl IntoResponse for ErrorPage {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
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
