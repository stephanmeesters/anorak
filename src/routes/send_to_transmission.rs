use crate::config::TRANSMISSION_URL;
use crate::models;

use anyhow::{Context, Result};
use axum::{
    extract::Path,
    http::{HeaderMap, HeaderValue},
    response::{IntoResponse, Json},
};
use reqwest::{header::CONTENT_TYPE, StatusCode};

pub async fn endpoint(Path(magnet): Path<String>) -> impl IntoResponse {
    let result = send_transmission_impl(magnet).await;
    match result {
        Ok(message) => (StatusCode::OK, Json(message)),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())),
    }
}

async fn send_transmission_impl(magnet: String) -> Result<String> {
    let request = prepare_transmission_req(magnet);
    let str = serde_json::to_string(&request)?;

    let client = reqwest::Client::new();
    let response = client.post(TRANSMISSION_URL).json(&str).send().await?;

    if response.status().as_u16() == 409 {
        let session_id = response
            .headers()
            .get("X-Transmission-Session-Id")
            .context("Transmission session id not found")?
            .to_str()?;

        let mut headers = HeaderMap::new();
        headers.insert(
            "X-Transmission-Session-Id",
            HeaderValue::from_str(session_id)?,
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let response = client
            .post(TRANSMISSION_URL)
            .headers(headers)
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
        method: "torrent-add".to_owned(),
        arguments: models::AddTorrentArguments {
            paused: false,
            filename: magnet,
        },
    }
}
