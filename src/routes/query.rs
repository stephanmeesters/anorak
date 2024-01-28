use crate::models;
use crate::config::{JACKETT_URL, JACKETT_APIKEY};
use crate::app_error::AppError;
use crate::ENV;

use axum::response::Html;
use axum::Json;
use axum::{
    response::IntoResponse,
    debug_handler,
};
use anyhow::Result;
use log::info;
use minijinja::{context, Value};
use serde_xml_rs::from_str;

#[debug_handler]
pub async fn endpoint(Json(payload):Json<models::Query>) -> Result<impl IntoResponse, AppError> {
    info!("{}", &payload.search_term);
    let items = gather_items_json(&payload.search_term).await?;
    let tmpl = ENV.get_template("query.html")?;
    let result = Html(tmpl.render(context!(items => items))?);
    Ok(result)
}

async fn gather_items_json(search_query: &str) -> Result<Value> {
    let contents = query_jackett(search_query).await?;
    let items:Vec<models::Item> = process_xml(&contents)?;
    // let str = serde_json::to_string(&items)?;
    //
    let contexts:Value = items.iter().map(|it| context! { title => it.title }).collect::<Vec<_>>().into();
    Ok(contexts)
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
