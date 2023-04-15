use std::{process,env};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Result error: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Result error: {}", e);
        process::exit(1);
    }
}
