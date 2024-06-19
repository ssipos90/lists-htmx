use hypertext::{html_elements, maud_move, Renderable};

use crate::common::ISO_DATE_TIME_FORMAT;

pub fn date_time<S: Into<String>>(
    format: S,
    date: chrono::NaiveDateTime,
) -> impl Renderable {
    let date_str = date.format(ISO_DATE_TIME_FORMAT).to_string();

    maud_move! {
        span data-format=(format.into()) { (&date_str) }
    }
}
