mod config;
mod models;
mod routes;

mod app_error;
use app_error::AppError;

use pretty_env_logger;
use axum::{
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use log::info;
use minijinja::{context, path_loader, Environment};
use once_cell::sync::Lazy;
use tower_http::services::ServeDir;

pub static ENV: Lazy<Environment<'static>> = Lazy::new(|| {
    let mut env = Environment::new();
    env.set_loader(path_loader("assets"));
    env
});

#[tokio::main]
pub async fn main() {
    pretty_env_logger::init();

    let app = Router::new()
        .route("/query", post(routes::query::endpoint))
        .route(
            "/send-to-transmission/:magnet",
            get(routes::send_to_transmission::endpoint),
        )
        .route("/test", get(endpoint))
        .nest_service("/", ServeDir::new("assets"));

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", config::PORT))
        .await
        .unwrap();
    info!("Anorak running on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn endpoint() -> Result<impl IntoResponse, AppError> {
    let tmpl = ENV.get_template("template.html")?;
    let result = Html(tmpl.render(context!(name => "John"))?);
    Ok(result)
}
