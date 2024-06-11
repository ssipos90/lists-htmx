use axum::{response::IntoResponse, routing::get, Router};
use hypertext::{maud, html_elements, GlobalAttributes};

use crate::layouts::default::DefaultLayout;

async fn home() -> impl IntoResponse {
    DefaultLayout::try_new(
        maud! {
            div {
                p class="lead" {
                  "Welcome to the home page!"
                }
            }
        }
    ).await
}

pub fn routes() -> Router {
    Router::new().route("/", get(home))
}
