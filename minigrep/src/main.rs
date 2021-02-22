// to accept command line arguments
use std::env;

// for aborting execution
use std::process;

// bringing our api into scope
use minigrep::Config;

fn main() {
    // args() is an iterator that returns the command line args as a series of values
    // collect() takes those values and puts them into our vector
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        
        // non-zero exit - indicates error
        process::exit(1);
    });

    println!("Searching for {}\nin file {}", config.query, config.filename);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

