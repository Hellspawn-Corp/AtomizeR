use crate::model::json::json_entries::JsonEntries;
use crate::model::json::json_entries::JsonEntry;
use crate::utils;
use chrono::{FixedOffset, Utc};

use log::debug;
use std::io::{self};
use uuid::Uuid;

/// Create new default template for JSON feed
pub fn create_feed_json_template() -> io::Result<()> {
    let template = JsonEntries {
        entries: vec![JsonEntry {
            id: Uuid::new_v4(),
						internal_id: "User-Defined-ID".to_string(),
            title: "Entry Title".to_string(),
            url: "https://www.youtube.com/watch?v=QH2-TGUlwu4".to_string(),
            published: Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap()),
            updated: Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap()),
            summary: "Test".to_string(),
            content: "Hello everybody, my name is Markiplier and today we're going to be playing SCP Containment Breach: My Little Pony.".to_string(),
            hash: "d41d8cd98f00b204e9800998ecf8427e".to_string(),
        }],
    };

    debug!("Creating JSON feed template: {:?}", template);
    // Write the template to a file
    utils::json_writer::write_json_to_file(&template, "entries.json", Some(false))?;

    Ok(())
}
