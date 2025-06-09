use std::ops::Index;

use crate::model::json::json_entry::JsonEntry;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct JsonEntries {
    pub entries: Vec<JsonEntry>,
}

impl JsonEntries {
    pub fn new(entries: Vec<JsonEntry>) -> Self {
        JsonEntries { entries }
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
