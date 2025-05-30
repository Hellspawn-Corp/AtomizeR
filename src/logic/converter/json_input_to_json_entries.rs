use chrono::Utc;
use uuid::Uuid;

use crate::model::json::json_entries::JsonEntries;
use crate::model::json::json_entry::JsonEntry;
use crate::model::json::json_input::InputEntry;

fn input_entry_to_json_entry(input: InputEntry) -> JsonEntry {
    JsonEntry {
        id: Uuid::new_v4(), // Generate a unique ID
        internal_id: input.id,
        title: input.title,
        url: input.url,
        published: input.published.unwrap_or_else(Utc::now).into(),
        updated: input.updated.unwrap_or_else(Utc::now).into(),
        summary: input.summary, // Default to now if not provided
        content: input.content,
        hash: "".to_owned(),
    }
}

pub fn convert_user_input_to_entries(user_inputs: Vec<InputEntry>) -> JsonEntries {
    user_inputs
        .into_iter()
        .map(input_entry_to_json_entry)
        .collect()
}
