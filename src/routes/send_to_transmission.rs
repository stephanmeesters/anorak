use std::fmt::Debug;

use crate::models;
use crate::{app_error::AppError, config::TRANSMISSION_URL};

use anyhow::{Context, Result};
use axum::debug_handler;
use axum::{extract::Path, response::IntoResponse};
use log::info;
use reqwest::header::CONTENT_TYPE;
use urlencoding::decode;

fn print_request(request: &reqwest::Request, body: &str) {
    info!(
        "Request\n\tMethod: {}\n\tURL: {}\n\tHeaders:{}\n\tBody: {}",
        request.method(),
        request.url(),
        request
            .headers()
            .iter()
            .map(|(key, value)| format!("{}: {:?}", key, value))
            .collect::<Vec<String>>()
            .join("\n"),
        body
    );
}

#[debug_handler]
pub async fn endpoint(Path(magnet): Path<String>) -> Result<impl IntoResponse, AppError> {
    let decoded = decode(&magnet)?.into_owned();
    let result = send_transmission_impl(decoded).await?;
    Ok(result)
}

async fn send_transmission_impl(magnet: String) -> Result<String> {
    let request = prepare_transmission_req(magnet);
    let str = serde_json::to_string(&request)?;

    let client = reqwest::Client::new();
    let response = client.post(TRANSMISSION_URL).json(&str).send().await?;

    let req = client.post(TRANSMISSION_URL).json(&str).build()?;
    print_request(&req, &str);

    if response.status().as_u16() == 409 {
        let session_id = response
            .headers()
            .get("X-Transmission-Session-Id")
            .context("Transmission session id not found")?
            .to_str()?;

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "X-Transmission-Session-Id",
            reqwest::header::HeaderValue::from_str(session_id)?,
        );
        // headers.insert(
        //     CONTENT_TYPE,
        //     reqwest::header::HeaderValue::from_static("application/json"),
        // );

        let req = client
            .post(TRANSMISSION_URL)
            .headers(headers.clone())
            .json(&str)
            .build()?;
        print_request(&req, &str);

        let response = client
            .post(TRANSMISSION_URL)
            .headers(headers.clone())
            .json(&str)
            .send()
            .await?;
        let txt = response.text().await?;
        return Ok(txt);
    }

    let txt = response.text().await?;
    Ok(txt)
}

fn prepare_transmission_req(magnet: String) -> models::TransmissionAddTorrent {
    models::TransmissionAddTorrent {
        method: "free-space".to_owned(),
        // method: "torrent-add".to_owned(),
        // arguments: models::AddTorrentArguments {
        //     paused: false,
        //     filename: magnet,
        // },
    }
}
