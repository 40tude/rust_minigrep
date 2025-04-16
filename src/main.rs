use std::env;
use std::fs;
struct Config {
    query: String,
    file_path: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // let (query, file_path) = parse_config(&args);
    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");
    println!("With text :\n{contents}");
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("Not enough arguments")
        }
        let query = args[1].clone(); // at this stage it is OK to .clone()
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}
