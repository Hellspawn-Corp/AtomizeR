use serde::Serialize;
use std::fs::File;
use std::io::Write;

pub fn write_json_to_file(json_to_write: &impl Serialize, file_path: &str) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(json_to_write).expect("Failed to serialize JSON data");

    let mut file = File::create(file_path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}
