use std::env;
use std::error::Error;
use std::fs;
use std::process;
fn main() {

    // Collect the command-line arguments
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

 // Extract the query and filename from the arguments
    let query = &args[1];
    let filename = &args[2];

    // Print the arguments to confirm they're correctly captured
    println!();
    println!("Query: {}", config.query);
    println!();
    println!("Filename: {}", config.filename);
    println!();

    if let Err(e) = run(config){
        println!("Application error : {}",e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?; 
    println!("With text:\n{}", contents);   
    Ok(())
}




struct Config{
    query: String,
    filename: String,
}

impl Config {

    fn new(args: &[String]) ->Result<Config, &str>{

        if args.len() < 3 {
            return Err( " Not enought Argument");
        }

        // Extract the query and filename from the arguments
        let query = args[1].clone();
        let filename = args[2].clone();
        
        Ok(Config {query, filename})
    
    }
    
}


