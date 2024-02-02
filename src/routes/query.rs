use crate::models;
use crate::config::CONFIG;
use crate::app_error::AppError;
use crate::ENV;
use crate::utils;

use axum::response::Html;
use axum::Form;
use axum::{
    response::IntoResponse,
    debug_handler,
};
use anyhow::Result;
use log::info;
use minijinja::{context, Value};
use serde_xml_rs::from_str;

#[debug_handler]
pub async fn endpoint(Form(payload):Form<models::Query>) -> Result<impl IntoResponse, AppError> {
    info!("{}", &payload.search_term);
    let items = gather_items_json(&payload.search_term).await?;
    let tmpl = ENV.get_template("query.html")?;
    let result = Html(tmpl.render(context!(items => items))?);
    Ok(result)
}

async fn gather_items_json(search_query: &str) -> Result<Value> {
    let contents = query_jackett(search_query).await?;
    let items:Vec<models::Item> = process_xml(&contents).unwrap_or_default();

    let contexts:Value = items.iter().map(|it| context! { 
        title => it.title,
        guid => it.guid,
        pub_date => utils::format_date(&it.pub_date),
        size => utils::format_bytes(it.size)
    }).collect::<Vec<_>>().into();
    Ok(contexts)
}

fn format_query_url(search_query: &str) -> String {
    format!("{}/api?apikey={}&t=search&q={}", CONFIG.jackett_url, CONFIG.jackett_apikey, search_query)
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
