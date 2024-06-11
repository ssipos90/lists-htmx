use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub type PageResult<T> = core::result::Result<T, ErrorPage>;

#[derive(Debug)]
pub enum ErrorPage {
}

impl IntoResponse for ErrorPage {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}
