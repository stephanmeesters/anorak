use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Rss {
    pub channel: Channel,
}

#[derive(Debug, Deserialize)]
pub struct Channel {
    pub item: Vec<Item>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    pub title: String,
    pub guid: String,
    pub comments: String,
    #[serde(rename = "pubDate")]
    pub pub_date: String,
    pub size: u64,
    pub files: u32,
    pub description: String,
    pub category: Vec<String>,
    #[serde(rename = "torznab:attr")]
    pub torznab_attrs: Option<Vec<TorznabAttr>>,    
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TorznabAttr {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@value")]
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TransmissionAddTorrent {
    pub method: String,
    // pub arguments: AddTorrentArguments,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AddTorrentArguments {
    pub paused: bool,
    pub filename: String,
}

#[derive(Deserialize)]
pub struct Query {
   pub search_term : String,
}

#[derive(Serialize, Deserialize)]
pub struct SendToTransmission {
   pub magnet : String,
}
