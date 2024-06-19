use axum::{
    http,
    response::{IntoResponse, Response},
};
use hypertext::{html_elements, maud, GlobalAttributes, Renderable};

pub struct DefaultLayout<R> {
    content: R,
    title: String,
    status: http::StatusCode,
}

impl<R: Renderable> DefaultLayout<R> {
    pub fn new<S: Into<String>>(title: S, content: R) -> Self {
        Self {
            content,
            title: title.into(),
            status: http::StatusCode::OK
        }
    }

    pub fn set_status(mut self, status: http::StatusCode) -> Self {
        self.status = status;
        self
    }
}

impl<R: Renderable> Renderable for DefaultLayout<R> {
    fn render_to(self, output: &mut String) {
        maud! {
            !DOCTYPE
            html lang="en" {
                head {
                    title { (self.title)}
                    script defer src="/js/htmx.min.js" {}
                    script defer src="/js/script.js" {}
                    link rel="stylesheet" href="/style.css";
                }
                body class="bg-gray-100" {
                    main id="content" class="container mx-auto my-10" {
                        (self.content)
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
