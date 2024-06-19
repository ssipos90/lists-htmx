use axum::{
    response::IntoResponse,
    routing::get,
    Router,
};
use hypertext::{maud, html_elements, GlobalAttributes};

use crate::layouts::default::DefaultLayout;

pub async fn about() -> impl IntoResponse {
    DefaultLayout::new(
        "About".to_string(),
        maud! {
            div {
                p class="lead" {
                  "Manele"
                }
            }
        }
    )
}
