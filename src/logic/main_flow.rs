use crate::cli::Cli;
use crate::logic::templating;
use env_logger::Env;
use log::{error, info};

pub fn start_flow(args: Cli) -> std::io::Result<()> {
    // Set log level based on --debug flag
    let log_level = if args.debug { "debug" } else { "info" };
    env_logger::Builder::from_env(Env::default().default_filter_or(log_level)).init();
    match args.entries {
        Some(json) => {
            info!("Using JSON file: {}", json);
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
