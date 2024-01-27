mod config;
mod models;
mod routes;

use axum::{response::{Html, IntoResponse}, routing::get, Router};
use tower_http::services::ServeDir;

#[tokio::main]
pub async fn main() {
    let app = Router::new()
        .route("/query/:query", get(routes::query::endpoint))
        .route(
            "/send-to-transmission/:magnet",
            get(routes::send_to_transmission::endpoint),
        )
        .route("/test", get(endpoint))
        .nest_service("/", ServeDir::new("assets"));

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", config::PORT))
        .await
        .unwrap();
    println!("Anorak running on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

pub async fn endpoint() -> impl IntoResponse {
    Html("<h1>Bla</h1>")
}
