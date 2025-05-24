use crate::model::json::json_entries::JsonEntries;
use crate::model::json::json_entry::JsonEntry;
use crate::utils;
use log::debug;
use std::io::{self};
use uuid::Uuid;

/// Create new default template for JSON feed
pub fn create_feed_json_template() -> io::Result<()> {
    let template = JsonEntries {
        entries: vec![JsonEntry {
            id: Uuid::new_v4().to_string(),
            title: "Entry Title".to_string(),
            content: "Hello everybody, my name is Markiplier and today we're going to be playing SCP Containment Breach: My Little Pony.".to_string(),
        }],
    };

    debug!("Creating JSON feed template: {:?}", template);
    // Write the template to a file
    utils::json_writer::write_json_to_file(&template, "entries.json", Some(false))?;

    Ok(())
}
