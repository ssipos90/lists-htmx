use hypertext::{html_elements, maud_move, GlobalAttributes, Renderable};

use crate::common::Message;

pub fn message(msg: Message) -> impl Renderable {
    maud_move! {
        @match msg {
            Message::Success(msg) => {
                div class="bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded relative my-2" {
                    strong class="font-bold" { "Success!" }
                    " "
                    span class="block sm:inline" { (msg) }
                }
            }
            Message::Error(msg) => {
                div class="bg-red-100 border border-red-400 text-green-700 px-4 py-3 rounded relative my-2" {
                    strong class="font-bold" { "Error!" }
                    " "
                    span class="block sm:inline" { (msg) }
                }
            }
        }
    }
}
