use log::{debug, error};
use serde::de::DeserializeOwned;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufReader};

use crate::model::json::json_input::{InputEntries, InputEntry};

pub fn read_json_from_file<T: DeserializeOwned>(file_path: &str) -> io::Result<T> {
    debug!("Reading JSON data from file: {}", file_path);
    debug!("Expected JSON object type: {}", std::any::type_name::<T>());
    let file = File::open(file_path).map_err(|e| {
        error!("Failed to open file '{}': {}", file_path, e);
        e
    })?;
    let reader = BufReader::new(file);

    let json_data = serde_json::from_reader(reader).map_err(|e| {
        error!(
            "Failed to deserialize JSON data from '{}': {}",
            file_path, e
        );
        io::Error::new(io::ErrorKind::InvalidData, e)
    })?;
    debug!("Successfully read JSON data from file: {}", file_path);
    Ok(json_data)
}

pub fn validate_input_json(entries: &InputEntries) -> io::Result<bool> {
    debug!("Reading and validating input JSON");

    let mut ids = HashSet::new();
    for entry in entries.entries.iter() {
        if !ids.insert(&entry.id) {
            error!("Duplicate ID found: {}", entry.id);
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Duplicate ID found: {}", entry.id),
            ));
        }
    }

    debug!("All IDs are unique in the input JSON file.");
    Ok(true)
}
