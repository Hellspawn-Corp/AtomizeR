use clap::Parser;

mod cli;
use cli::Cli;
mod logic {
    pub mod main_flow;
    pub mod templating;
    pub mod converter {
        pub mod json_input_to_json_entries;
        pub mod json_to_atom;
    }
}
mod utils {
    pub mod atom_reader;
    pub mod atom_writer;
    pub mod json_reader;
    pub mod json_writer;
    mod tests;
}
mod model {
    pub mod base {
        pub mod entry;
        pub mod feed;
    }
    pub mod json {
        pub mod json_entries;
        pub mod json_input;
    }
    pub mod atom {
        pub mod atom_entry;
    }
}

fn main() -> std::io::Result<()> {
    let args: Cli = Cli::parse();
    logic::main_flow::start_flow(args)
}
