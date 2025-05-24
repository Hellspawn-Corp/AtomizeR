use log::{debug, error};
use serde::Serialize;
use std::fs::OpenOptions;
use std::io::{self, Write};

pub fn write_json_to_file<T: Serialize + std::fmt::Debug>(
    json_to_write: &T,
    file_path: &str,
    overwrite: Option<bool>,
) -> io::Result<()> {
    let overwrite = overwrite.unwrap_or(true); // Default to true

    debug!("Writing JSON data to file: {}", file_path);
    debug!("JSON object type: {}", std::any::type_name::<T>());
    debug!("JSON data to write: {:?}", json_to_write);

    let json = serde_json::to_string_pretty(json_to_write).map_err(|e| {
        error!("Failed to serialize JSON data: {}", e);
        io::Error::new(io::ErrorKind::Other, e)
    })?;

    let file_result = if overwrite {
        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)
    } else {
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(file_path)
    };

    let mut file = file_result.map_err(|e| {
        if !overwrite && e.kind() == io::ErrorKind::AlreadyExists {
            error!("File '{}' already exists! Preventing overwrite.", file_path);
        } else {
            error!("Failed to create or open file '{}': {}", file_path, e);
        }
        e
    })?;

    file.write_all(json.as_bytes())?;

    Ok(())
}
