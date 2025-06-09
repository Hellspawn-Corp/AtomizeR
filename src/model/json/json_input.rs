use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, de::DeserializeOwned};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct InputEntry {
    pub id: String,
    pub title: String,
    pub url: String,
    pub summary: String,
    pub content: String,
    pub published: Option<DateTime<Utc>>,
    pub updated: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InputEntries {
    pub entries: Vec<InputEntry>,
}

impl InputEntries {
    pub fn new(entries: Vec<InputEntry>) -> Self {
        InputEntries { entries }
    }
}

impl IntoIterator for InputEntries {
    type Item = InputEntry;
    type IntoIter = std::vec::IntoIter<InputEntry>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}
