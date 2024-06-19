use hypertext::{maud, html_elements, GlobalAttributes, Renderable};

pub fn not_found() -> impl Renderable {
    maud!(
        div class="bg-gray-100 flex items-center justify-center" {
            div class="max-w-md p-6 bg-white rounded-lg shadow-md" {
                h1 class="text-2xl font-semibold mb-4 text-center text-gray-800" { "404 Not Found" }
                p class="text-gray-600 text-center" { "The page you are looking for does not exist." }
            }
        }
    )
}
