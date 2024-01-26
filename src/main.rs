mod models; 
mod routes;
mod config;

use std::net::SocketAddr;
use axum::{ routing::get, Router };

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/query/:query", get(routes::query::endpoint))
        .route("/send-to-transmission/:magnet", get(routes::send_to_transmission::endpoint));

    let addr = SocketAddr::from(([0,0,0,0], 9341));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
