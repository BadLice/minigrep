use std::{error::Error, fs};

pub fn run(config: Config) -> Result<Config, Box<dyn Error>> {
    let contents: String = read_file(&config)?;
    let matches = search_case_sensitive(&config, &contents);
    print_matches(&matches);
    Ok(config)
}

fn read_file(config: &Config) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;
    Ok(contents)
}

type Match<'a> = Vec<(&'a str, &'a str, &'a str)>;
fn search_case_sensitive<'a>(config: &Config, contents: &'a str) -> Match<'a> {
    let mut matches: Match<'a> = Vec::new();
    for line in contents.lines() {
        let indexes = line.match_indices(&config.query);

        for (i, found) in indexes {
            let prev = &line[..i];
            let next = &line[i + found.len()..];
            matches.push((prev, found, next));
        }
    }
    matches
}

fn search_case_insensitive<'a>(config: &Config, contents: &'a str) -> Match<'a> {
    let mut matches: Match<'a> = Vec::new();
    let query = config.query.to_lowercase();
    for line in contents.lines() {
        let indexes = line.to_lowercase().match_indices(&query);
        for (i, found) in indexes {
            let prev = &line[..i];
            let next = &line[i + found.len()..];
            matches.push((prev, found, next));
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

pub struct Config {
    file_path: String,
    query: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        // 3 args in total: default exec path + 2 user arguments
        if args.len() != 3 {
            return Err("2 arguments needed");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { file_path, query })
    }
}
