use crate::model::json::json_entry::JsonEntry;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct JsonEntries {
    pub entries: Vec<JsonEntry>,
}
