// to access filesystem
use std::fs;

// for error handling
use std::error::Error;

// for environment variables
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool, // optional environment variable
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }        
        
        // clone trades off performance for simplicity. We'll go with simple for now.
        let query = args[1].clone();
        let filename = args[2].clone();

        // to set the env variable, run $Env:CASE_INSENSITIVE=1 in powershell
        // run Remove-Item Env:CASE_INSENSITIVE to remove the env variable
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { 
            query,
            filename,
            case_sensitive,
        })
    }
}

// () is the unit type, Box<dyn Error> is a trait object
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? at the end of the method call returns the error value to the caller 
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    
    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    
    result
}

// case-insensitive search
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();

    for line in contents.lines() {
        // to_lowercase() creates new data instead of refering to old
        // contains expects a string slice, hence the &query
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}


// unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "Hell";
        let contents = "\
Rust is nice.
Hello Rust.
No hellfire, only peace.";

        assert_eq!(vec!["Hello Rust."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUSt";
        let contents = "\
Rust is nice.
Hello trusty Rust.";

        assert_eq!(
            vec!["Rust is nice.", "Hello trusty Rust."],
            search_case_insensitive(query, contents)
        );
    }
}