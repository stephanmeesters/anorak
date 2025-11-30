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
pub struct Attr {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    pub title: String,
    pub guid: String,
    #[serde(default)]
    pub comments: String,
    #[serde(rename = "pubDate")]
    pub pub_date: String,
    #[serde(default)]
    pub size: u64,
    #[serde(default)]
    pub files: u32,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub category: Vec<String>,
    #[serde(rename = "attr", default)]
    pub attrs: Vec<Attr>,
}

#[derive(Deserialize)]
pub struct Query {
   pub search_term : String,
}

#[derive(Serialize, Deserialize)]
pub struct SendToTransmission {
   pub magnet : String,
}
