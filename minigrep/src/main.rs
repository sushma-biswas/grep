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

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        // printing error messages to std error instead of std output
        eprintln!("Problem parsing arguments: {}", err);
        
        // non-zero exit - indicates error
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}

