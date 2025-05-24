use log::{debug, error};
use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::{self, BufReader};

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
