use axum::{
    response::IntoResponse,
    routing::get,
    Router,
};
use hypertext::{maud, html_elements, GlobalAttributes};

use crate::layouts::default::DefaultLayout;

async fn about() -> impl IntoResponse {
    DefaultLayout::try_new(
        maud! {
            div {
                p class="lead" {
                  "Manele"
                }
            }
        }
    ).await
}

pub fn routes() -> Router {
    Router::new().route("/about", get(about))
}
