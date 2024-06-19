use axum::{http, response::IntoResponse};
use hypertext::{html_elements, maud, GlobalAttributes};

use crate::layouts::default::DefaultLayout;

pub async fn not_found() -> impl IntoResponse {
    // make sure you're using tailwind typography
    DefaultLayout::new(
        "404 Not Found",
        maud! {
            div class="bg-gray-100 flex items-center justify-center" {
                div class="max-w-md p-6 bg-white rounded-lg shadow-md" {
                    h1 class="text-2xl font-semibold mb-4 text-center text-gray-800" { "404 Not Found" }
                    p class="text-gray-600 text-center" { "The page you are looking for does not exist." }
                }
            }
        },
    )
    .set_status(http::StatusCode::NOT_FOUND)
}
