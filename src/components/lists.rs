use hypertext::{html_elements, maud, maud_move, GlobalAttributes, Renderable};

use crate::{
    common::{Message, PaginatedResult, DATE_TIME_FORMAT, ISO_DATE_TIME_FORMAT}, components, htmx::HtmxAttributes, models
};

pub fn create_list(message: Option<Message>) -> impl Renderable {
    maud_move! {
        div class="bg-white shadow-md rounded-lg p-6" {
            h1 class="text-xl font-semibold mb-4" { "Create a new list" }
            @if let Some(message) = message {
                (message)
            }
            form id="todo-form" hx-post="/list" method="post" hx-target="#content" hx-push-url="true" {
                div class="mb-4" {
                    label class="block text gray-700 text-sm font-bold mb-2" for="name" { "Name" }
                    input type="text"
                          name="name"
                          placeholder="Name"
                          class="w-full px-4 py-2 mr-2 rounded-lg border-gray-300 focus:outline-none focus:border-blue-500"
                          required;
                }
            }
        }
    }
}

pub fn lists(
    result: PaginatedResult<models::SlimList>,
    message: Option<Message>,
) -> impl Renderable {
    let prev_link = (result.page > 1).then(|| {
        format!(
            "/list?page={}&items_per_page={}",
            result.page - 1,
            result.items_per_page
        )
    });
    let next_link = (result.total_pages > result.page).then(|| {
        format!(
            "/list?page={}&items_per_page={}",
            result.page + 1,
            result.items_per_page
        )
    });

    maud_move! {
        h1 class="text-center text-3xl font-semibold mb-4" {
            span {"Lists" }
        }
        div class="md:w-1/2 mx-auto" {
            div class="bg-white shadow-md rounded-lg p-6" {
                @if let Some(message) = message {
                    (message)
                }

                form id="todo-form" hx-post="/list" method="post" hx-target="#content" hx-push-url="true" {
                    div class="flex mb-4" {
                        input type="text"
                              name="name"
                              placeholder="Start a new checklist"
                              class="w-full px-4 py-2 mr-2 rounded-lg border-gray-300 focus:outline-none focus:border-blue-500"
                              required;
                        button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" type="submit" {
                            "Add"
                        }
                    }
                }

                section class="list-none p-0" {
                    @for list in result.items {
                        a href=(format!("/list/{}", list.id)) class="flex border-t border-gray-100 hover:bg-gray-100 p-4" {
                           div class="w-10 h-8 flex h-full column items-center justify-center" {
                               div class="w-4 h-4 bg-blue-500" {}
                           }
                           div class="flex-1" {
                                h4 class="text-blue-500 font-medium" {
                                    (list.name)
                                }

                                div class="text-gray-400 text-xs" {
                                    "Created: "
                                    (components::date_time(DATE_TIME_FORMAT, list.created_at))
                                }
                                @if let Some(completed) = list.completed_at {
                                    div class="text-gray-400 text-xs" {
                                        (components::date_time(DATE_TIME_FORMAT, completed))
                                    }
                                }
                                div class="text-gray-400 text-xs" {
                                    (components::date_time(DATE_TIME_FORMAT, list.updated_at))
                                }
                            }
                        }
                    }
                }
            }


            div class="text-sm text-gray-700 flex justify-between mt-4" {
                div class="text-left flex-grow w-1/3" {
                    @if let Some(prev_link) = prev_link {
                        a
                            href=(&prev_link)
                            hx-get=(&prev_link)
                            hx-target="#todo-list"
                            hx-push-url="true"
                        { "< Previous page" }
                    }
                }

                div class ="text-center w-1/3" {
                    @if result.total_pages > 1 {
                        span { "Page " (result.page) " of " (result.total_pages) }
                    }
                }

                div class="text-right flex-grow w-1/3" {
                    @if let Some(next_link) = next_link {
                        a
                            href=(&next_link)
                            hx-get=(&next_link)
                            hx-target="#todo-list"
                            hx-push-url="true"
                        { "Next page >" }
                    }
                }
            }
        }
    }
}

pub fn list(list: models::List) -> impl Renderable {
    maud_move! {
        div class="bg-white shadow-md rounded-lg p-6 mb-4" {
            h1 class="text-xl font-semibold mb-2" { (list.name) }
            p class="text-gray-500 mb-2" { "Created at: " (list.created_at.format(DATE_TIME_FORMAT).to_string()) }
            p class="text-gray-500 mb-2" { "Updated at: " (list.updated_at.format(DATE_TIME_FORMAT).to_string()) }
            @if let Some(completed_at) = list.completed_at {
                p class="text-gray-500 mb-2" { "Completed at: " (completed_at.format(DATE_TIME_FORMAT).to_string()) }
            }
            @if let Some(description) = list.description {
                p class="text-gray-700" { (description) }
            }
            @else {
                p class="text-gray-500" { "No description provided." }
            }
        }
    }
}
