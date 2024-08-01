use std::env;
use std::process;
use rusty_cli::Config;

fn main() {
    // Collect the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Parse the configuration from command-line arguments
    let config = match Config::new(&args) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        }
    };

    // Optional: Print the arguments to confirm they're correctly captured
    // Remove these lines in production to avoid unnecessary output
    println!();
    println!("Query: {}", config.query);
    println!();
    println!("Filename: {}", config.filename);
    println!();

    // Run the application logic
    if let Err(e) = rusty_cli::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
