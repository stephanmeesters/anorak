use std::net::SocketAddr;

use axum::{
    routing::get,
    Router, response::{IntoResponse, Json},
    debug_handler,
};
use anyhow::Result;
use reqwest::StatusCode;
use serde::{Serialize, Deserialize};
use serde_xml_rs::from_str;

static JACKETT_URL: &str = "http://192.168.2.10:9117/api/v2.0/indexers/thepiratebay/results/torznab";
static JACKETT_APIKEY: &str = "qjr3cgynmdxzlgwpdjv3x5pwxohzx3zi";

#[derive(Debug, Deserialize)]
struct Rss {
    channel: Channel,
}

#[derive(Debug, Deserialize)]
struct Channel {
    item: Vec<Item>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Item {
    title: String,
    guid: String,
    comments: String,
    #[serde(rename = "pubDate")]
    pub_date: String,
    size: u64,
    files: u32,
    description: String,
    category: Vec<String>,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(content));

    let addr = SocketAddr::from(([0,0,0,0], 9341));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[debug_handler]
async fn content() -> impl IntoResponse {
    let items = gather_items_json("Evangelion").await;
    match items {
        Ok(content) => (StatusCode::OK, Json(content)),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string()))
    }
}

async fn gather_items_json(search_query: &str) -> Result<String> {
    let contents = query_jackett("Evangelion").await?;
    let items = process_xml(&contents);
    serde_json::to_string(&items)
}

fn format_query_url(search_query: &str) -> String {
    format!("{}/api?apikey={}&t=search&q={}", JACKETT_URL, JACKETT_APIKEY, search_query)
}

async fn query_jackett(search_query: &str) -> Result<String> {
    let response = reqwest::get(format_query_url(search_query)).await?;
    let body = response.text().await?;
    Ok(body)
}

fn process_xml(xml: &str) -> Result<Vec<Item>> {
    let rss: Rss = from_str(xml.trim())?;
    rss.channel.item
}
