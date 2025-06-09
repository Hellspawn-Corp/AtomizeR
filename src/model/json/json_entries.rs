use std::ops::Index;

use crate::model::json::json_entries::JsonEntry;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonEntries {
    pub entries: Vec<JsonEntry>,
}

impl JsonEntries {
    pub fn new(entries: Vec<JsonEntry>) -> Self {
        JsonEntries { entries }
    }

    pub fn push(&mut self, entry: JsonEntry) {
        self.entries.push(entry);
    }
}

impl Iterator for JsonEntries {
    type Item = JsonEntry;

    fn next(&mut self) -> Option<Self::Item> {
        self.entries.pop()
    }
}

impl FromIterator<JsonEntry> for JsonEntries {
    fn from_iter<I: IntoIterator<Item = JsonEntry>>(iter: I) -> Self {
        let entries: Vec<JsonEntry> = iter.into_iter().collect();
        JsonEntries { entries }
    }
}

impl ExactSizeIterator for JsonEntries {
    fn len(&self) -> usize {
        self.entries.len()
    }
}

impl Index<usize> for JsonEntries {
    type Output = JsonEntry;

    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index]
    }
}

impl Extend<JsonEntry> for JsonEntries {
    fn extend<T: IntoIterator<Item = JsonEntry>>(&mut self, iter: T) {
        self.entries.extend(iter);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonEntry {
    pub id: Uuid,
    pub internal_id: String,
    pub title: String,
    pub url: String,
    pub published: DateTime<FixedOffset>,
    pub updated: DateTime<FixedOffset>,
    pub summary: String,
    pub content: String,
    pub hash: String,
}
