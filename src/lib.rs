use std::{env, error::Error, fs};

pub fn run(config: Config) -> Result<Config, Box<dyn Error>> {
    let contents: String = read_file(&config)?;
    let matches = if config.ignore_case {
        search_case_insensitive(&config, &contents)
    } else {
        search_case_sensitive(&config, &contents)
    };
    print_matches(&matches);
    Ok(config)
}

fn read_file(config: &Config) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;
    Ok(contents)
}

type Match<'a> = Vec<(&'a str, String, &'a str)>;
fn search_case_sensitive<'a>(config: &Config, contents: &'a str) -> Match<'a> {
    let mut matches: Match<'a> = Vec::new();
    for line in contents.lines() {
        let indexes = line.match_indices(&config.query);
        for (i, found) in indexes {
            let prev = &line[..i];
            let next = &line[i + found.len()..];
            matches.push((prev, found.to_owned(), next));
        }
    }
    matches
}

fn search_case_insensitive<'a>(config: &Config, contents: &'a str) -> Match<'a> {
    let mut matches: Match<'a> = Vec::new();
    let query = config.query.to_lowercase();
    for line in contents.lines() {
        let line_lowercased = line.to_lowercase();
        let indexes = line_lowercased.match_indices(&query);
        for (i, found) in indexes {
            let prev = &line[..i];
            let next = &line[i + found.len()..];
            matches.push((prev, found.to_owned(), next));
        }
    }
    matches
}

fn print_matches(matches: &Match) {
    for (prev, found, next) in matches {
        print!("{}", prev);
        print!("\x1b[31m{}\x1b[0m", found);
        println!("{}", next);
    }
}
#[derive(Debug)]
pub struct Config {
    file_path: String,
    query: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // skip first arg as it is the exec path in every program by default
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Query string not defined"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("File path not defined"),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            file_path,
            query,
            ignore_case,
        })
    }
}
