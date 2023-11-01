use std::{error::Error, fs};

pub fn run(config: Config) -> Result<Config, Box<dyn Error>> {
    let mut lines: Vec<String> = Vec::new();
    let mut matches: Vec<(String, String, String)> = Vec::new();
    read_file(&config, &mut lines)?;
    search(&config, &lines, &mut matches);
    print_matches(&matches);
    Ok(config)
}

fn read_file(config: &Config, lines: &mut Vec<String>) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(&config.file_path)?;
    *lines = contents
        .split('\n')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    Ok(())
}

fn search(config: &Config, lines: &Vec<String>, matches: &mut Vec<(String, String, String)>) {
    for line in lines {
        let indexes = line.match_indices(&config.query);

        for (i, found) in indexes {
            let prev = &line[..i];

            let next = &line[i + found.len()..];
            matches.push((prev.to_string(), found.to_string(), next.to_string()));
        }
    }
}

fn print_matches(matches: &Vec<(String, String, String)>) {
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
