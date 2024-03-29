use crate::models::SendToTransmission;
use crate::{app_error::AppError, config::CONFIG};

use anyhow::{anyhow, Result};
use axum::{debug_handler, Form};
use axum::response::{Html, IntoResponse};
use transmission_rpc::types::TorrentAddArgs;
use transmission_rpc::TransClient;
use urlencoding::decode;

#[debug_handler]
pub async fn endpoint(
    Form(magnet): Form<SendToTransmission>,
) -> Result<impl IntoResponse, AppError> {
    let decoded = decode(&magnet.magnet)?.into_owned();
    send_transmission_impl(decoded).await?;
    Ok(Html("<span class=\"fa-solid fa-check\"></span>"))
}

async fn send_transmission_impl(magnet: String) -> Result<String> {
    let mut client = TransClient::new(CONFIG.transmission_url.parse()?);
    let add: TorrentAddArgs = TorrentAddArgs {
        filename: Some(magnet),
        ..TorrentAddArgs::default()
    };
    match client.torrent_add(add).await {
        Ok(res) => Ok(res.result),
        Err(e) => Err(anyhow!(e))
    }
}
