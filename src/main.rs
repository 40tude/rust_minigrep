use std::env;
use std::error::Error;
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

    if let Err(e) = run(config) {
        println!("Application error : {e}");
        process::exit(1);
    }
}

// In case of Ok case, the function still return unit type ()
// For error type, use the trait object Box<dyn Error> (see chap 17)
// run will return a type that implement the Error trait but we don't specify the particular type
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text :\n{contents}");
    Ok(()) // wrap the unit type () in the OK()
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
