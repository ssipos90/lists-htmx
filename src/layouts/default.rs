use axum::{
    http,
    response::{IntoResponse, Response},
};
use hypertext::{html_elements, maud, GlobalAttributes, Renderable};

use crate::htmx::HtmxAttributes;

pub struct DefaultLayout<R> {
    content: R,
    some_stuff_from_db: String,
}

impl<R: Renderable> DefaultLayout<R> {
    pub async fn try_new(content: R) -> Result<Self, String> {
        Ok(Self {
            content,
            some_stuff_from_db: "some stuff".to_string(),
        })
    }
}

impl<R: Renderable> Renderable for DefaultLayout<R> {
    fn render_to(self, output: &mut String) {
        maud! {
            !DOCTYPE
            html lang="en" {
                head {
                    title { "About" }
                }
                body {
                    h1 { "About" }
                    section class="container" {
                        (self.content)
                    }
                    p { "Some stuff from the database: " (self.some_stuff_from_db) }
                    div hx_get="/api/some-data" hx_target="#some-data" {
                        "Click here to load some data"
                    }
                }
            }
        }
        .render_to(output);
    }
}

impl<R: Renderable> IntoResponse for DefaultLayout<R> {
    fn into_response(self) -> Response {
        (http::StatusCode::OK, self.render()).into_response()
    }
}
