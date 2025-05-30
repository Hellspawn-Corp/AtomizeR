use crate::model::json::json_entry::JsonEntry;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct JsonEntries {
    pub entries: Vec<JsonEntry>,
}

impl FromIterator<JsonEntry> for JsonEntries {
    fn from_iter<I: IntoIterator<Item = JsonEntry>>(iter: I) -> Self {
        let entries: Vec<JsonEntry> = iter.into_iter().collect();
        JsonEntries { entries }
    }
}
