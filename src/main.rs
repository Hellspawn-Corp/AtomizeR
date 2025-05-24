mod cli;
use clap::Parser;
use cli::Cli;

fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    match args.json {
        Some(json) => println!("Using JSON file: {}", json),
        None => println!("No file provided, creating default template posts.json!"),
    }
    Ok(())
    // if let Some(path) = args.json {
    //     std::fs::write(&path, "test").expect("Failed to write JSON file");
    //     println!("Greeting written to {}", path);
    // } else {
    //     println!("Hello, world!");
    // }
}
