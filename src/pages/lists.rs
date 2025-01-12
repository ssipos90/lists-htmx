use axum::{
    extract::{Form, Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Redirect},
};
use hypertext::Renderable;
use std::sync::Arc;

use crate::{
    common::{Message, Pagination, QueryPagination},
    components,
    error::{ErrorPage, PageResult},
    layouts::default::DefaultLayout,
    models, services, AppState,
};

#[tracing::instrument(skip(state))]
pub async fn create_list(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    Form(form): Form<models::CreateList>,
) -> impl IntoResponse {
    let (msg, status) = match services::create_list(&state.pool, form).await {
        Ok(list) => {
            return Redirect::to(&format!("/list/{}", list.id)).into_response();
        }
        Err(_) => (
            Message::Error("Failed to create list".into()),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
    };

    let component = components::create_list(Some(msg));

    dbg!(&headers);

    if headers.get("HX-Request").is_some() {
        (status, component.render()).into_response()
    } else {
        DefaultLayout::new("Create list", component)
            .set_status(status)
            .into_response()
    }
}

#[tracing::instrument(skip(state))]
pub async fn lists(
    State(state): State<Arc<AppState>>,
    Query(pagination): Query<QueryPagination>,
) -> PageResult<impl IntoResponse> {
    let pagination: Pagination = pagination.into();

    let lists = services::fetch_lists(&state.pool, pagination)
        .await
        .unwrap();

    Ok(DefaultLayout::new(
        format!("Lists (Page {})", lists.page),
        components::lists(lists, None),
    ))
}

#[tracing::instrument(skip(state))]
pub async fn list(
    State(state): State<Arc<AppState>>,
    Path(list_id): Path<i32>,
) -> PageResult<impl IntoResponse> {
    let list = services::fetch_list(&state.pool, list_id).await.unwrap();

    match list {
        Some(list) => Ok(DefaultLayout::new(
            format!("List: {}", list.name),
            components::list(list),
        )),
        None => Err(ErrorPage::NotFound(Some("List not found".into()))),
    }
}
