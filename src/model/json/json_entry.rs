use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct JsonEntry {
    pub id: String,
    pub title: String,
    pub content: String,
}
