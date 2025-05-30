use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct InputEntry {
    pub id: String,
    pub title: String,
    pub url: String,
    pub summary: String,
    pub content: String,
    pub published: Option<DateTime<Utc>>,
    pub updated: Option<DateTime<Utc>>,
}
