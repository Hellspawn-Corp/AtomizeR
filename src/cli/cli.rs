use clap::Parser;

/// Clap parser
#[derive(Parser)]
pub struct Cli {
    /// Input JSON file with entries to be parsed into an Atom feed.
    /// If not provided, the default is "entries.json"
    /// If this is your first time running this, leave this empty and a template "entries.json" will be created
    #[arg(short, long, value_name = "entries_json_input_file")]
    pub entries: Option<String>,
    /// Output file path for the Atom feed.
    /// If not provided, the default is "feed.xml"
    #[arg(short, long, value_name = "output_feed_file_path")]
    pub output_feed: Option<String>,
    /// Enable debug logging
    #[arg(short, long)]
    pub debug: bool,
}
