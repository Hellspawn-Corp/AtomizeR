mod test {
    use std::io;

    use serde::de;

    use crate::model::json::json_input::{InputEntries, InputEntry};

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
        let entries = crate::utils::json_reader::read_json_from_file("input.json")
            .expect("Should read JSON file.");

        let result = crate::utils::json_reader::validate_input_json(&entries);
        assert!(result.is_err());
        assert_eq!(
            Err(io::ErrorKind::InvalidData),
            result.map_err(|e| e.kind())
        );
    }

    #[test]
    fn it_panics_if_json_not_valid() {
        crate::utils::json_reader::read_json_from_file::<InputEntries>(
            "tests/res/invalid_json.json",
        )
        .expect_err("This should have panicked due to invalid JSON");
    }

    #[test]
    fn it_panics_if_file_cant_be_read() {
        crate::utils::json_reader::read_json_from_file::<Vec<InputEntry>>("does_not_exist.json")
            .expect_err("This should have panicked due to file not found");
    }
}
