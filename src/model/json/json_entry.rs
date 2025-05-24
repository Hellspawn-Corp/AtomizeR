use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Serialize, Deserialize, Debug)]
pub struct JsonEntry {
    pub id: Uuid,
    pub title: String,
    pub url: String,
    pub published: DateTime<FixedOffset>,
    pub updated: DateTime<FixedOffset>,
    pub summary: String,
    pub content: String,
    pub hash: String,
}
