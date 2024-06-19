use axum::extract::State;
use axum::handler::HandlerWithoutStateExt;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, get_service};
use axum::Router;
use dotenvy::dotenv;
use error::ApiResult;
use std::sync::Arc;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

use crate::pages::not_found;

mod common;
mod error;
mod htmx;
mod layouts;
mod models;
mod pages;
mod components;
mod services;

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::PgPool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must set");
    let pool = sqlx::PgPool::connect_lazy(&database_url).expect("Failed to connect to Postgres");

    // build our application with a route
    let all_routes = Router::new()
        .route("/health_check", get(health_check))
        .route("/", get(pages::dashboard))
        .route(
            "/list",
            get(pages::lists).post(pages::create_list),
        )
        .route("/list/:list_id", get(pages::list))
        .route("/about", get(pages::about))
        .with_state(Arc::new(AppState { pool }))
        .fallback_service(ServeDir::new("public").not_found_service(not_found.into_service()))
        .layer(TraceLayer::new_for_http());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, all_routes).await.unwrap();
}

async fn health_check(State(state): State<Arc<AppState>>) -> ApiResult<Response> {
    state
        .pool
        .acquire()
        .await
        .map(|_| (StatusCode::OK, "OK").into_response())
        .map_err(error::ApiError::DatabaseError)
}
