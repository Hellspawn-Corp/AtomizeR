mod test {
    use crate::logic::main_flow::update_feed;
    use crate::model::json::{
        json_entries::{JsonEntries, JsonEntry},
        json_input::{InputEntries, InputEntry},
    };
    use chrono::{DateTime, FixedOffset};
    use serde::de;
    use std::io;
    use uuid::Uuid;

    #[test]
    fn it_parses_json_input() {
        let entries: Vec<InputEntry> =
            crate::utils::json_reader::read_json_from_file("tests/res/test.json").unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].id, "dumb-id");
        assert_eq!(entries[0].title, "Entry Title");
        assert_eq!(entries[0].url, "https://www.url.com/");
        assert_eq!(entries[0].summary, "Test");
        assert_eq!(
            entries[0].content,
            "Hello everybody, my name is Markiplier and today we're going to be playing SCP Containment Breach: My Little Pony."
        );
    }

    #[test]
    fn it_panics_on_non_unique_id() {
        let json: Vec<InputEntry> =
            crate::utils::json_reader::read_json_from_file("tests/res/stinky_data_test.json")
                .expect("Should read JSON file.");

        let wrapped_entries = InputEntries::new(json);

        let result = crate::utils::json_reader::validate_input_json(&wrapped_entries);
        assert!(result.is_err());
        assert_eq!(
            Err(io::ErrorKind::InvalidData),
            result.map_err(|e| e.kind())
        );
    }

    #[test]
    fn it_panics_if_json_not_valid() {
        crate::utils::json_reader::read_json_from_file::<Vec<InputEntry>>(
            "tests/res/invalid_json.json",
        )
        .expect_err("This should have panicked due to invalid JSON");
    }

    #[test]
    fn it_panics_if_file_cant_be_read() {
        crate::utils::json_reader::read_json_from_file::<Vec<InputEntry>>("does_not_exist.json")
            .expect_err("This should have panicked due to file not found");
    }

    #[test]
    fn update_feed_updated_with_changes_ok() {
        // Prepare test data
        let uuid_entry_1 = Uuid::new_v4();
        let uuid_entry_2 = Uuid::new_v4();

        let old_updated_entry_1: DateTime<FixedOffset> = chrono::Utc::now().into();
        let old_updated_entry_2: DateTime<FixedOffset> = chrono::Utc::now().into();

        let old_published_entry_1: DateTime<FixedOffset> = chrono::Utc::now().into();
        let old_published_entry_2: DateTime<FixedOffset> = chrono::Utc::now().into();
        // GIVEN
        // Old entries file with correct data exists
        // AND
        // New entries file with updated contents is provided
        let old_json = JsonEntries::new(Vec::from([
            JsonEntry {
                id: uuid_entry_1,
                internal_id: "old-id-1".to_string(),
                title: "Old Entry 1".to_string(),
                url: "https://example.com/old1".to_string(),
                published: old_published_entry_1,
                updated: old_updated_entry_1,
                summary: "Old summary 1".to_string(),
                content: "Old content 1".to_string(),
                hash: "hash0X".to_string(),
            },
            JsonEntry {
                id: uuid_entry_2,
                internal_id: "old-id-2".to_string(),
                title: "Old Entry 2".to_string(),
                url: "https://example.com/old2".to_string(),
                published: old_published_entry_2,
                updated: old_updated_entry_2,
                summary: "Old summary 2".to_string(),
                content: "Old content 2".to_string(),
                hash: "hashGX".to_string(),
            },
        ]));
        let new_json = JsonEntries::new(Vec::from([
            JsonEntry {
                id: uuid_entry_1,
                internal_id: "old-id-1".to_string(),
                title: "Markimoo".to_string(),
                url: "https://github.com".to_string(),
                published: old_published_entry_1,
                updated: old_updated_entry_1,
                summary: "THE END IS NEAR".to_string(),
                content: "yea we caught a live one".to_string(),
                hash: "hash666".to_string(),
            },
            JsonEntry {
                id: uuid_entry_2,
                internal_id: "old-id-2".to_string(),
                title: "Pewdiepie".to_string(),
                url: "https://gitlab.com".to_string(),
                published: old_published_entry_2,
                updated: old_updated_entry_2,
                summary: "Old summary 2".to_string(),
                content: "Old content 2".to_string(),
                hash: "hash999".to_string(),
            },
            JsonEntry {
                id: Uuid::new_v4(),
                internal_id: "brand-new-id-3".to_string(),
                title: "JackSepticEye".to_string(),
                url: "https://codeberg.com".to_string(),
                published: chrono::Utc::now().into(),
                updated: chrono::Utc::now().into(),
                summary: "Subnautica let's play".to_string(),
                content: "Slop".to_string(),
                hash: "hash420".to_string(),
            },
        ]));

        // WHEN
        // User tries to update the feed with new entries
        let updated_old_entries = update_feed(&old_json, &new_json);

        // THEN
        let first_entry = &updated_old_entries[0];
        let second_entry = &updated_old_entries[1];
        let third_entry = &updated_old_entries[2];
        // Existing entries are kept
        assert_eq!(first_entry.internal_id, old_json.entries[0].internal_id);
        assert_eq!(second_entry.internal_id, old_json.entries[1].internal_id);
        // New entries are added
        assert_eq!(third_entry.internal_id, new_json.entries[2].internal_id);
    }
}
