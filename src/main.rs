mod config;
mod models;
mod routes;
mod utils;

mod app_error;
use axum::{
    routing::post,
    Router,
};
use log::info;
use minijinja::{path_loader, Environment};
use once_cell::sync::Lazy;
use pretty_env_logger;
use tower_http::services::ServeDir;

use crate::config::CONFIG;

pub static ENV: Lazy<Environment<'static>> = Lazy::new(|| {
    let mut env = Environment::new();
    env.set_loader(path_loader("assets"));
    env
});

#[tokio::main]
pub async fn main() {
    pretty_env_logger::init();

    let app = Router::new()
        .route("/query/", post(routes::query::endpoint))
        .route(
            "/send-to-transmission/",
            post(routes::send_to_transmission::endpoint),
        )
        .nest_service("/", ServeDir::new("assets"));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", CONFIG.port))
        .await
        .unwrap();
    info!("Anorak running on http://localhost:{}", CONFIG.port);
    axum::serve(listener, app).await.unwrap();
}
