use crate::cli::Cli;
use crate::logic::converter::json_to_atom;
use crate::logic::templating;
use crate::model::json::json_entries;
use crate::model::json::json_entries::JsonEntries;
use crate::model::json::json_entry;
use crate::model::json::json_feed;
use crate::utils::atom_writer;
use crate::utils::json_reader;
use env_logger::Env;
use log::{debug, error, info};

pub fn start_flow(args: Cli) -> std::io::Result<()> {
    // Set log level based on --debug flag
    let log_level = if args.debug { "debug" } else { "info" };
    env_logger::Builder::from_env(Env::default().default_filter_or(log_level)).init();
    match args.entries {
        Some(json) => {
            info!("Using JSON file: {}", json);
            // process json
            let json_entries: JsonEntries = match json_reader::read_json_from_file(&json) {
                Ok(entries) => entries,
                Err(e) => {
                    error!("Failed to read JSON file: {}", e);
                    return Err(e);
                }
            };
            let atom_output_path = args
                .output_feed
                .clone()
                .unwrap_or_else(|| "feed.xml".to_string());
            // if output path is provided, write atom feed to that path

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
// fn start_flow() {
//     // Initialize logging
//     env_logger::init();

//     // Load configuration
//     let config = match load_config() {
//         Ok(cfg) => cfg,
//         Err(e) => {
//             eprintln!("Error loading configuration: {}", e);
//             return;
//         }
//     };

//     // Initialize the templating engine
//     let template_engine = match init_template_engine(&config.template_path) {
//         Ok(engine) => engine,
//         Err(e) => {
//             eprintln!("Error initializing template engine: {}", e);
//             return;
//         }
//     };

//     // Start the main application logic
//     if let Err(e) = run_application(&config, &template_engine) {
//         eprintln!("Application error: {}", e);
//     }
// }
