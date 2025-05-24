use clap::Parser;

/// Clap parser
#[derive(Parser)]
pub struct Cli {
    /// Input JSON file with posts to be parsed into an Atom feed.
    /// If not provided, the default is "posts.json"
    /// If this is your first time running this, leave this empty and a template "posts.json" will be created
    #[arg(short, long, value_name = "json_input_file")]
    pub json: Option<String>,
}
