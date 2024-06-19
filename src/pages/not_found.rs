use axum::{http, response::IntoResponse};

use crate::{components, layouts::default::DefaultLayout};

pub async fn not_found() -> impl IntoResponse {
    DefaultLayout::new(
        "404 Not Found",
        components::not_found(),
    )
    .set_status(http::StatusCode::NOT_FOUND)
}
