use std::net::SocketAddr;
use axum::{
    routing::get,
    Router, response::{IntoResponse, Json},
    debug_handler, extract::Path, http::{HeaderMap, HeaderValue},
};
use anyhow::{Result, Context};
use reqwest::{StatusCode, header::CONTENT_TYPE};
use serde::{Serialize, Deserialize};
use serde_xml_rs::from_str;

static JACKETT_URL: &str = "http://192.168.2.10:9117/api/v2.0/indexers/thepiratebay/results/torznab";
static JACKETT_APIKEY: &str = "qjr3cgynmdxzlgwpdjv3x5pwxohzx3zi";
static TRANSMISSION_URL: &str = "http://192.168.2.10:9091/transmission/rpc";

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

#[derive(Debug, Deserialize, Serialize)]
struct TransmissionAddTorrent {
    method: String,
    arguments: AddTorrentArguments,
}

#[derive(Debug, Deserialize, Serialize)]
struct AddTorrentArguments {
    paused: bool,
    filename: String
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/query/:query", get(query_endpoint))
        .route("/send-to-transmission/:magnet", get(send_transmission_endpoint));

    let addr = SocketAddr::from(([0,0,0,0], 9341));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Query

#[debug_handler]
async fn query_endpoint(Path(query):Path<String>) -> impl IntoResponse {
    let items = gather_items_json(&query).await;
    match items {
        Ok(content) => (StatusCode::OK, Json(content)),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string()))
    }
}

async fn gather_items_json(search_query: &str) -> Result<String> {
    let contents = query_jackett(search_query).await?;
    let items = process_xml(&contents)?;
    let str = serde_json::to_string(&items)?;
    Ok(str)
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
    Ok(rss.channel.item)
}

// Send to Transmission

async fn send_transmission_endpoint(Path(magnet):Path<String>) -> impl IntoResponse {
    // let result = send_transmission_impl(magnet).await;
    // match result {
    //     Ok(message) => (StatusCode::OK, Json(message)),
    //     Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string()))
    // }
    Json(magnet)
}

async fn send_transmission_impl(magnet: String) -> Result<String> {
    let request = prepare_transmission_req(magnet);
    let str = serde_json::to_string(&request)?;

    let client = reqwest::Client::new();
    let response = client.post(TRANSMISSION_URL).json(&str).send().await?;

if response.status().as_u16() == 409 {
        let session_id = response.headers()
            .get("X-Transmission-Session-Id").context("Transmission session id not found")?.to_str()?;

        let mut headers = HeaderMap::new();
        headers.insert("X-Transmission-Session-Id", HeaderValue::from_str(session_id)?);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let response = client.post(TRANSMISSION_URL).headers(headers).json(&str).send().await?;
    let txt = response.text().await?;
    return Ok(txt);
}

    let txt = response.text().await?;
    Ok(txt)
}

fn prepare_transmission_req(magnet: String) -> TransmissionAddTorrent {
    TransmissionAddTorrent { 
        method: "torrent-add".to_owned(),
        arguments: AddTorrentArguments {
            paused: false,
            filename: magnet
        }
    }
}
