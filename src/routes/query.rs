use crate::models;
use crate::config::{JACKETT_URL, JACKETT_APIKEY};
use crate::app_error::AppError;

use axum::{
    response::{IntoResponse},
    debug_handler, extract::Path,
};
use anyhow::Result;
use serde_xml_rs::from_str;

#[debug_handler]
pub async fn endpoint(Path(query):Path<String>) -> Result<impl IntoResponse, AppError> {
    let items = gather_items_json(&query).await?;
    Ok(items)
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

fn process_xml(xml: &str) -> Result<Vec<models::Item>> {
    let rss: models::Rss = from_str(xml.trim())?;
    Ok(rss.channel.item)
}
