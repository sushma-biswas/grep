// to access filesystem
use std::fs;

// for error handling
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }        
        
        // clone trades off performance for simplicity. We'll go with simple for now.
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

// () is the unit type, Box<dyn Error> is a trait object
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? at the end of the method call returns the error value to the caller 
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}