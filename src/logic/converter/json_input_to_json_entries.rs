use chrono::Utc;
use uuid::Uuid;

use crate::model::json::json_entries::JsonEntries;
use crate::model::json::json_entries::JsonEntry;
use crate::model::json::json_input::{InputEntries, InputEntry};

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

pub fn convert_user_input_to_entries(user_inputs: InputEntries) -> JsonEntries {
    user_inputs
        .into_iter()
        .map(input_entry_to_json_entry)
        .collect()
}

mod test {
    use crate::{
        logic::converter::json_input_to_json_entries::convert_user_input_to_entries,
        model::json::json_input::InputEntries,
    };

    #[test]
    fn test_convert_user_input_to_entries() {
        let input = crate::model::json::json_input::InputEntry {
            id: "test-id".to_string(),
            title: "Test Entry".to_string(),
            url: "https://example.com".to_string(),
            published: None,
            updated: None,
            summary: "This is a test entry.".to_string(),
            content: "Content of the test entry.".to_string(),
        };

        let entries = convert_user_input_to_entries(InputEntries::new(vec![input]));
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].internal_id, "test-id");
        assert_eq!(entries[0].title, "Test Entry");
        assert_eq!(entries[0].url, "https://example.com");
    }
}
