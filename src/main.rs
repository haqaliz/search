use std::env;
use std::process;

use search::Params;

fn main() {
    let params = Params::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = search::run(params) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}

