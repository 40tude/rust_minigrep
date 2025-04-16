use std::env;
use std::fs;
use std::process;
struct Config {
    query: String,
    file_path: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // if Result is an OK value, .unwrap_or_else() returns the inner value wrapped by Ok
    // On Err value, the closure gets called
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments : {err}"); // see the content of Err in Config::build()
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    run(config);
}

fn run(config: Config) {
    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");
    println!("With text :\n{contents}");
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone(); // at this stage it is OK to .clone()
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
