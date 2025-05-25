use atom_syndication::Feed;
use log::{debug, error};
use std::fs::File;
use std::io::{self, Write};

pub fn write_atom_feed_to_file(feed: &Feed, file_path: &str) -> io::Result<()> {
    debug!("Writing Atom feed to file: {}", file_path);

    let xml = feed.to_string();

    let mut file = match File::create(file_path) {
        Ok(f) => f,
        Err(e) => {
            error!("Failed to create file '{}': {}", file_path, e);
            return Err(e);
        }
    };

    if let Err(e) = file.write_all(xml.as_bytes()) {
        error!("Failed to write Atom feed to '{}': {}", file_path, e);
        return Err(e);
    }

    debug!("Atom feed successfully written to {}", file_path);
    Ok(())
}
