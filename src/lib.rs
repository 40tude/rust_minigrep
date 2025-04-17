// $Env:IGNORE_CASE=1; cargo run -- to poem.txt
// Remove-Item Env:IGNORE_CASE

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, content)
        );
    }
}

use std::env;
use std::error::Error;
use std::fs;

// pub is everywhere...
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    // pub fn build(args: &[String]) -> Result<Config, &'static str> {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // name of the program

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Did'nt get a query string."),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Did'nt get a file path."),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

// In case of Ok case, the function still return unit type ()
// For error type, use the trait object Box<dyn Error> (see chap 17)
// run will return a type that implement the Error trait but we don't specify the particular type
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(()) // wrap the unit type () in the OK()
}

// explicit lifetime is mandatory to specify that the return value is a string slice that references
// slices of the argument contents (not query)
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();

    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    // results
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        // .contains() wait for a reference
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}
