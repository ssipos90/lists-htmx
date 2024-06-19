use axum::response::IntoResponse;
use hypertext::{maud, html_elements, GlobalAttributes};

use crate::layouts::default::DefaultLayout;

pub async fn dashboard() -> impl IntoResponse {
    DefaultLayout::new(
        "Home",
        maud! {
            div {
                p class="lead" {
                  "Welcome to the home page!"
                }
            }
        }
    )
}
