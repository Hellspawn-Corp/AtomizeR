use clap::Parser;

mod cli;
use cli::Cli;
mod logic {
    pub mod main_flow;
    pub mod templating;
}
mod utils {
    pub mod json_reader;
    pub mod json_writer;
}

mod model {
    pub mod json {
        pub mod json_entries;
        pub mod json_entry;
        pub mod json_feed;
    }
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    logic::main_flow::start_flow(args)
}
