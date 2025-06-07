mod test {

    #[test]
    fn it_parses_json_input() {
        let entries =
            crate::utils::json_reader::read_and_validate_input_json("tests/res/test.json").unwrap();
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
        let result = crate::utils::json_reader::read_and_validate_input_json(
            "tests/res/stinky_data_test.json",
        );
        result.expect_err("This should have panicked due to non-unique ID");
    }

    #[test]
    fn it_panics_if_json_not_valid() {
        let result =
            crate::utils::json_reader::read_and_validate_input_json("tests/res/invalid_json.json");
        result.expect_err("This should have panicked due to invalid JSON format");
    }
}
