use std::env;
use std::process;

use minigrep::Config; // bring the Config type from the lib crate into the binary scope

fn main() {
    let args: Vec<String> = env::args().collect();

    // if Result is an OK value, .unwrap_or_else() returns the inner value wrapped by Ok
    // On Err value, the closure gets called
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments : {err}"); // see the content of Err in Config::build()
        process::exit(1);
    });

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);

    // prefix the run with lib crate name
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error : {e}");
        process::exit(1);
    }
}
