use crate::cli::Cli;
use crate::logic::converter::json_input_to_json_entries::convert_user_input_to_entries;
use crate::logic::converter::json_to_atom;
use crate::logic::templating;
use crate::utils::atom_writer;
use crate::utils::json_reader::read_and_validate_input_json;
use crate::utils::json_writer::write_json_to_file;
use env_logger::Env;
use log::{debug, error, info};

pub fn start_flow(args: Cli) -> std::io::Result<()> {
    // Set log level based on --debug flag
    let log_level = if args.debug { "debug" } else { "info" };
    env_logger::Builder::from_env(Env::default().default_filter_or(log_level)).init();
    match args.entries {
        Some(json) => {
            info!("Using JSON file: {}", json);

            let user_input = match read_and_validate_input_json(&json) {
                Ok(entries) => {
                    println!("Successfully read and validated input.json:");
                    for entry in &entries {
                        println!("{:?}", entry);
                    }
                    entries
                }
                Err(e) => {
                    error!("Error reading or validating input.json: {:?}", e);
                    return Err(e);
                }
            };

            let json_entries = convert_user_input_to_entries(user_input);

            write_json_to_file(&json_entries, "entries.json", Some(true))?;

            let atom_output_path = args
                .output_feed
                .clone()
                .unwrap_or_else(|| "feed.xml".to_string());

            info!("Writing Atom feed to: {}", atom_output_path);
            let atom_feed = json_to_atom::json_entries_to_atom(&json_entries.entries);
            match atom_writer::write_atom_feed_to_file(&atom_feed, &atom_output_path) {
                Ok(_) => info!("Atom feed successfully written to {}", atom_output_path),
                Err(e) => {
                    error!("Failed to write Atom feed: {}", e);
                    return Err(e);
                }
            }

            let atom_feed = json_to_atom::json_entries_to_atom(&json_entries.entries);
            debug!("converted json to atom feed: {:?}", atom_feed);
        }
        None => {
            info!("No file provided, creating default template entries.json!");
            let result: Result<(), std::io::Error> = templating::create_feed_json_template();

            if let Err(e) = result {
                error!("Error occurred: {}", e);
            }
        }
    }

    Ok(())
}
