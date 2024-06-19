use hypertext::html_elements;
use hypertext::{maud, Renderable};

pub fn not_found() -> impl Renderable {
    maud!(
        div {
            h1 { "404 Not Found" }
            p { "The page you are looking for does not exist." }
        }
    )
}
