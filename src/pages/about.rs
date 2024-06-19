use axum::response::IntoResponse;
use hypertext::{maud, html_elements, GlobalAttributes};

use crate::layouts::default::DefaultLayout;

pub async fn about() -> impl IntoResponse {
    DefaultLayout::new(
        "About",
        maud! {
            div {
                p class="lead" {
                  "Manele"
                }
            }
        }
    )
}
