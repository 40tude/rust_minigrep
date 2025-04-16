#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }
}

use std::error::Error;
use std::fs;

// pub is everywhere...
pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone(); // at this stage it is OK to .clone()
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

// In case of Ok case, the function still return unit type ()
// For error type, use the trait object Box<dyn Error> (see chap 17)
// run will return a type that implement the Error trait but we don't specify the particular type
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    // println!("With text :\n{contents}");
    Ok(()) // wrap the unit type () in the OK()
}

// explicite lifetime is mandatory to specify that the return value is a string slice that references
// slices of the argument contents (not query)
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}
