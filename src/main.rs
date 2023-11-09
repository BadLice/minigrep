use grep::{run, Config};
use std::{env::args, process};

fn main() {
    let args: Vec<String> = args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("A problem with args occurred: {}", err);
        process::exit(1);
    });

    run(config).unwrap_or_else(|err| {
        eprintln!("A problem occurred: {}", err);
        process::exit(1);
    });
}
