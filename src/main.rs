use axum::routing::get_service;
use axum::Router;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

mod error;
mod pages;
mod layouts;
mod htmx;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // build our application with a route
    let all_routes = Router::new()
        .merge(pages::home::routes())
        .merge(pages::about::routes())
        .fallback_service(Router::new().nest_service("/", get_service(ServeDir::new("./"))))
        .layer(TraceLayer::new_for_http());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, all_routes).await.unwrap();
}
