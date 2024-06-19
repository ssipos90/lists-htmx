use hypertext::Renderable;
use serde::Deserialize;

use crate::components::message;

pub const DATE_TIME_FORMAT: &str = "%c";

#[derive(Debug, Deserialize)]
pub struct QueryPagination {
    pub page: Option<i32>,
    pub items_per_page: Option<i32>,
}

impl From<QueryPagination> for Pagination {
    fn from(val: QueryPagination) -> Self {
        Pagination {
            page: val.page.unwrap_or(1),
            items_per_page: val.items_per_page.unwrap_or(10),
        }
    }
}

pub struct Pagination {
    pub page: i32,
    pub items_per_page: i32,
}

impl Default for Pagination {
    fn default() -> Self {
        Pagination {
            page: 1,
            items_per_page: 10,
        }
    }
}

pub struct PaginatedResult<T> {
    pub items: Vec<T>,
    pub page: i32,
    pub total_pages: i32,
    pub items_per_page: i32,
}

pub enum Message {
    Success(String),
    Error(String),
}

impl Renderable for Message {
    fn render_to(self, output: &mut String) {
        message(self).render_to(output);
    }
}
