
use crate::app_error::AppError;
use crate::config::CONFIG;
use crate::models;
use crate::utils;
use crate::ENV;

use anyhow::{anyhow, Result};
use axum::response::Html;
use axum::Form;
use axum::{debug_handler, response::IntoResponse};
use log::info;
use minijinja::{context, Value};
use serde_xml_rs::from_str;
use transmission_rpc::types::TorrentGetField;
use transmission_rpc::TransClient;

#[debug_handler]
pub async fn endpoint(Form(payload): Form<models::Query>) -> Result<impl IntoResponse, AppError> {
    info!("{}", &payload.search_term);
    let items = gather_items_json(&payload.search_term).await?;
    let tmpl = ENV.get_template("query.html")?;
    let result = Html(tmpl.render(context!(items => items))?);
    Ok(result)
}

async fn gather_items_json(search_query: &str) -> Result<Value> {
    let contents = query_jackett(search_query).await?;
    let mut items: Vec<models::Item> = process_xml(&contents).unwrap_or_default();
    let known_torrents = request_transmission_known_torrents().await?;

    let attr_as_u32 = |attrs: &[models::Attr], name: &str| -> u32 {
        attrs
            .iter()
            .find(|a| a.name == name)
            .and_then(|a| a.value.parse::<u32>().ok())
            .unwrap_or(0)
    };

    // Default sort: highest seeders first, then peers.
    items.sort_by(|a, b| {
        let a_seeders = attr_as_u32(&a.attrs, "seeders");
        let b_seeders = attr_as_u32(&b.attrs, "seeders");
        b_seeders
            .cmp(&a_seeders)
            .then_with(|| attr_as_u32(&b.attrs, "peers").cmp(&attr_as_u32(&a.attrs, "peers")))
    });

    let contexts: Value = items
        .iter()
        .map(|it| {
            let seeders = attr_as_u32(&it.attrs, "seeders");
            let peers = attr_as_u32(&it.attrs, "peers");
            context! {
                already_added => known_torrents.iter().any(|t| t.contains(&it.title)),
                title => it.title,
                guid => it.guid,
                seeders => seeders,
                peers => peers,
                pub_date => utils::format_date_unix(&it.pub_date),
                pub_date_format => utils::format_date(&it.pub_date),
                size => it.size,
                size_format => utils::format_bytes(it.size),
            }
        })
        .collect::<Vec<_>>()
        .into();
    Ok(contexts)
}

async fn request_transmission_known_torrents() -> Result<Vec<String>> {
    let mut client = TransClient::new(CONFIG.transmission_url.parse()?);
    let fields = vec![TorrentGetField::Name];
    match client.torrent_get(Some(fields), None).await {
        Ok(res) => Ok(res.arguments.torrents.into_iter().map(|t| t.name.unwrap_or_default()).collect()),
        Err(e) => Err(anyhow!(e)),
    }
}

fn format_query_url(search_query: &str) -> String {
    format!(
        "{}/api?apikey={}&t=search&q={}",
        CONFIG.jackett_url, CONFIG.jackett_apikey, search_query
    )
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
