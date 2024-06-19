use hypertext::{html_elements, maud_move, GlobalAttributes, Renderable};

pub fn error<'a>(title: &'a str, details: Option<&'a str>) -> impl Renderable + 'a {
    maud_move!(
        div class="bg-red-100 flex items-center justify-center" {
            div class="max-w-md p-6 bg-white rounded-lg shadow-md" {
                h1 class="text-2xl font-semibold mb-4 text-center text-red-800" { (title) }
                @if let Some(details) = details {
                    p class="text-red-600 text-center" { (details) }
                }
            }
        }
    )
}
