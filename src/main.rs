use std::{env, process};

use replacio::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("From directory \"{}\"", config.dir_path);
    println!("Searching for \"{}\"", config.query);
    println!("Replacing with \"{}\"", config.replacement_text);

    if let Err(e) = replacio::run(config) {
        eprintln!("Application error: ${e}");
        process::exit(1);
    }
}
